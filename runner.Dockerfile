FROM debian:stable-slim
# for development: 'tesseract-ocr-all' includes all testdata
# because the runner shouldn't be oversized, we only install the required 
#    packages for running the executable, which are the testdata/language files
RUN apt update && apt install -y tesseract-ocr-eng tesseract-ocr-deu \
    # these libs bloat the image from 200 to around 800
    libopencv-superres4.5 libopencv-viz4.5

# libopencv-imgproc4.5 
# libopencv-core4.5 libopencv-imgcodecs4.5 
# libopencv-viz4.5 libopencv-dnn4.5 
# libopencv-flann4.5 libopencv-stitching4.5
# libopencv-features2d4.5 libopencv-objdetect4.5
# libopencv-photo4.5 libopencv-shape4.5 
# libopencv-calib3d4.5 libopencv-contrib4.5 
