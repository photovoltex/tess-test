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
    echo "container '$container_name' available, restarting container"
    docker container restart $container_name
    docker attach $container_name
fi