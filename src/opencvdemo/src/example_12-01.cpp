// Example 12-1. Using cv::dft() and cv::idft() to accelerate
// the computation of convolutions 
//利用傅里叶变换卷积和利用核游走整个图像进行卷积运算的区别:
//一般求法中，利用核游走整个图像进行卷积运算，实际上进行的是相关运算，真正意义上的卷积，应该首先把核翻转180度，再在整个图像上进行游走。OpenCV中的filter2D实际上做的也只是相关，而非卷积。

#include <iostream>
#include <opencv2/opencv.hpp>
#include "windows.h"
#include <stdio.h>

using std::cout;
using std::endl;

using namespace std;
using namespace cv;

int main2(int argc, char** argv) {
    if (argc != 2) {
        cout    << "\nExample 12-1. Using cv::dft() and cv::idft() to accelerate the"
                << "\n computation of convolutions"
                << "\nFourier Transform\nUsage: "
                << argv[0] << " <path/imagename>\n" << endl;
        return -1;
    }

    cv::Mat A = cv::imread(argv[1], 0);

    if (A.empty()) {
        cout << "Cannot load " << argv[1] << endl;
        return -1;
    }

    cv::Size patchSize(100, 100);
    cv::Point topleft(A.cols / 2, A.rows /2);
    cv::Rect roi(topleft.x, topleft.y, patchSize.width, patchSize.height);
    cv::Mat B = A(roi);

    int dft_M = cv::getOptimalDFTSize(A.rows + B.rows - 1);
    int dft_N = cv::getOptimalDFTSize(A.cols + B.cols - 1);

    cv::Mat dft_A = cv::Mat::zeros(dft_M, dft_N, CV_32F);
    cv::Mat dft_B = cv::Mat::zeros(dft_M, dft_N, CV_32F);

    cv::Mat dft_A_part = dft_A(cv::Rect(0, 0, A.cols, A.rows));
    cv::Mat dft_B_part = dft_B(cv::Rect(0, 0, B.cols, B.rows));

    A.convertTo(dft_A_part, dft_A_part.type(), 1, -mean(A)[0]);
    B.convertTo(dft_B_part, dft_B_part.type(), 1, -mean(B)[0]);

    cv::dft(dft_A, dft_A, 0, A.rows);
    cv::dft(dft_B, dft_B, 0, B.rows);

    // set the last parameter to false to compute convolution instead of correlation
    //
    cv::mulSpectrums(dft_A, dft_B, dft_A, 0, true);
    cv::idft(dft_A, dft_A, cv::DFT_SCALE, A.rows + B.rows - 1);

    cv::Mat corr = dft_A(cv::Rect(0, 0, A.cols + B.cols - 1, A.rows + B.rows - 1));
    cv::normalize(corr, corr, 0, 1, cv::NORM_MINMAX, corr.type());
    cv::pow(corr, 3.0, corr);

    B ^= cv::Scalar::all(255);

    cv::imshow("Image", A);
    cv::imshow("ROI", B);

    cv::imshow("Correlation", corr);
    cv::waitKey();

    return 0;
}

void convolveDFT(Mat& A,Mat& B, Mat& C)
{
  //【1】初始化输出矩阵
  C.create(abs(A.rows - B.rows) + 1, abs(A.cols - B.cols) + 1, A.type());
  Size dftSize; //???什么意思
  //【2】计算DFT变换的尺寸
  dftSize.width = getOptimalDFTSize(A.cols + B.cols - 1);
  dftSize.height = getOptimalDFTSize(A.rows + B.rows - 1);
  //【3】分配临时缓冲区并初始化置零
  Mat tempA(dftSize, A.type(), Scalar::all(0));
  Mat tempB(dftSize, B.type(), Scalar::all(0));
  //【4】分别复制A和B到tempA和tempB的左上角
  Mat roiA(tempA, Rect(0, 0, A.cols, A.rows));
  A.copyTo(roiA);
  Mat roiB(tempB, Rect(0, 0, B.cols, B.rows));
  B.copyTo(roiB);
  //【5】就地操作，进行快速傅里叶变换，并将nonzeroRows参数置为非零，以进行更加快速的处理???为什么
  dft(tempA, tempA, 0, A.rows);
  dft(tempB, tempB, 0, B.rows);
  //【6】将得到的频谱相乘，结果存放于tempA中
  mulSpectrums(tempA,tempB,tempA, DFT_COMPLEX_OUTPUT);//DFT_REAL_OUTPUT
  //【7】将结果变换为频域且尽管行结果都为非零，我们只需要其中C.rows的第一行，所以采用nonzeroRows==C.rows
  dft(tempA, tempA, DFT_INVERSE + DFT_SCALE, C.rows);
  //【8】将结果复制到C中
  tempA(Rect(0, 0, C.cols, C.rows)).copyTo(C);
  //所有的临时缓冲区将被自动释放，所以无须收尾操作
}

int main(int argc, char** argv)
{
  system("chcp 65001");
  
  SetConsoleTextAttribute(GetStdHandle(STD_OUTPUT_HANDLE), FOREGROUND_INTENSITY | FOREGROUND_GREEN);    //字体为绿色
  //载入原图
  Mat srcImage = imread(string("E:/app/julia/wfs2map/src/opencvvideo/")+string("tests/1.png"), 0); //读取灰度图
  Mat kernel = (Mat_<float>(3, 3) << 1, 1, 1, 1, 1, 1, 1, 1, 1);
  cout << kernel;
  Mat floatI = Mat_<float>(srcImage);// change image type into float
  Mat filteredI;
  convolveDFT(floatI, kernel, filteredI);
  normalize(filteredI, filteredI, 0, 1,NORM_MINMAX); // Transform the matrix with float values into a
                      // viewable image form (float between values 0 and 1).
  //imshow("image", srcImage);
  imshow("filtered", filteredI);    //这里显示报错，但是可以用ImageWatch查看，暂时不知道原因
  waitKey(0);
  return 0;
}
