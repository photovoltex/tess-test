# used for the image and the container later on
builder_name="rust-tess-builder"
runner_name="rust-tess-runner"

# dockerfiles used for the builder and runner images
builder_dockerfile="builder.Dockerfile"
runner_dockerfile="runner.Dockerfile"

# modes availabel: "build", "run" or "linting"
if [ "$1" != "" ]
then
    mode=$1
else 
    mode="run"
fi
# is used in mode "build" and "run"
configuration="release"
bin="tess-test"

# build the image, if the dockerfile was modified in the last n minutes
last_modified_in=3

# commands used for each mode
build_command="cargo build --$configuration"
linting_command="bacon clippy"
run_command="./target/$configuration/$bin"

# build builder img, when related Dockerfile was modified or image is not available
if [ "$(find $builder_dockerfile -type f -mmin -$last_modified_in)" != "" ] || 
   [ "$(docker images --filter "reference=*$builder_name*" -q)" = "" ]
then
    docker buildx build -t $builder_name \
                        --build-arg USER_ID=$(id -u) \
                        --build-arg GROUP_ID=$(id -g) \
                        -f $builder_dockerfile .
else 
    echo "skip building builder, builder image available"
fi

# build runner, when related Dockerfile was modified or image is not available
if [ "$(find $runner_dockerfile -type f -mmin -$last_modified_in)" != "" ] || 
   [ "$(docker images --filter "reference=*$runner_name*" -q)" = "" ]
then
    docker buildx build -t $runner_name -f $runner_dockerfile .
else
    echo "skip building runner, runner image available"
fi

if [ "$mode" = "build" ]
then
    ./docker_wrapper.sh $builder_name $mode "$build_command"
else
    if [ "$mode" = "run" ]
    then
        ./docker_wrapper.sh $builder_name "build" "$build_command"
        ./docker_wrapper.sh $runner_name $mode "$run_command"
    else 
        if [ "$mode" = "linting" ]
        then
            ./docker_wrapper.sh $builder_name $mode "$linting_command"
        else
            echo "mode: '"$mode"' is not a valid mode"
        fi
    fi
fi
