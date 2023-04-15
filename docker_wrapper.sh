image=$1
container_name="$2-$image"
command=$3

echo "executing '$command' in '$image'"

# run crate
if [ "$(docker container ls -a --filter "name=$container_name" -q)" = "" ]
then
    docker run -it \
        -v "$PWD":/app \
        -w /app \
        --name $container_name \
        -u $(id -u ${USER}):$(id -g ${USER}) \
        $image \
        $command
else
    # need to separate between build/run (most of the time to fast to attach) and linting
    if [ "$2" = "linting" ]
    then 
        echo "restarting '$(docker container restart $container_name)'"
        docker attach $container_name
    else
        echo "restarting '$container_name'"
        docker start $container_name --attach
    fi
fi