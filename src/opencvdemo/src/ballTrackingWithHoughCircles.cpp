/* EKREM SONMEZER - 2021 */

#include <opencv2/opencv.hpp> 
#include <iostream>
#include <cmath>

using namespace cv;
using namespace std;


int main() {

    VideoCapture cap;

    //if (!cap.open(0))
    if (!cap.open("E:/app/julia/wfs2map/src/opencvvideo/tests/ball_tracking_example.mp4"))
    {
        cout << "Webcam not connected.\n" << "Please verify\n";
        return EXIT_FAILURE;
    }

    while (true)
    {
        Mat frame, gray;

        //get frame from capture
        cap >> frame;
        
        // to grayscale
        cvtColor(frame, gray, COLOR_BGR2GRAY);

        // smooth it, otherwise a lot of false circles may be detected
        GaussianBlur(gray, gray, Size(11, 11),2,2);
    
        vector<Vec3f> circles;

        //change this parameters
        HoughCircles(gray, circles, HOUGH_GRADIENT, 0.8, 10,50,30,30,40);

        for (size_t i = 0; i < circles.size(); i++)
        {
            Point center(cvRound(circles[i][0]), cvRound(circles[i][1]));
            int radius = cvRound(circles[i][2]);
            // draw the circle center
            circle(frame, center, 3, Scalar(255, 0, 0), -1, 8, 0);
            // draw the circle outline
            circle(frame, center, radius, Scalar(0, 0, 255), 3, 8, 0);
        }


        if (waitKey(30) == 27 || frame.empty())
            break;

        imshow("Webcam", frame);
    }
    return 0;
}