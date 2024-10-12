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

#include <iconv.h>

int GbkToUtf8(const char *src_str, size_t src_len, char *dst_str, size_t dst_len)
{
  iconv_t cd;
  char **pin = nullptr;
  *pin = const_cast<char*>(src_str);
  char **pout = &dst_str;

  cd = iconv_open("utf8", "gbk");
  if (cd == 0)
    return -1;
  memset(dst_str, 0, dst_len);
  if (iconv(cd, pin, &src_len, pout, &dst_len) == -1)
    return -1;
  iconv_close(cd);
  *pout = '\0';

  return 0;
}

int Utf8ToGbk(const char *src_str, size_t src_len, char *dst_str, size_t dst_len)
{
  iconv_t cd;
  char **pin = nullptr;
  *pin = const_cast<char*>(src_str);
  char **pout = &dst_str;

  cd = iconv_open("gbk", "utf8");
  if (cd == 0)
    return -1;
  memset(dst_str, 0, dst_len);
  if (iconv(cd, pin, &src_len, pout, &dst_len) == -1)
    return -1;
  iconv_close(cd);
  *pout = '\0';

  return 0;
}

int main()
{
  system("chcp 65001");

  SetConsoleTextAttribute(GetStdHandle(STD_OUTPUT_HANDLE), FOREGROUND_INTENSITY | FOREGROUND_GREEN);    //字体为绿色
  //1、载入原图
  Mat srcImage = imread(string("E:/app/julia/wfs2map/src/opencvvideo/")+string("tests/1.png"), 0); //读取灰度图
  //2、将图像扩大到合适的尺寸（当图像的尺寸是2.3.5的整数倍时，运行速度最快）
  //【2】将输入图像延扩到最佳尺寸，边界用0补充
  int m = getOptimalDFTSize(srcImage.rows);
  int n = getOptimalDFTSize(srcImage.cols);
  //将添加的像素初始化为0
  Mat padded;
  copyMakeBorder(srcImage, padded, 0, m - srcImage.rows, n - srcImage.cols, BORDER_CONSTANT,0);
  //3、为傅里叶变换的结果（实部和虚部）分配存储空间
  Mat planes[] = { Mat_<float>(padded),Mat::zeros(padded.size(),CV_32F) };
  Mat complexI;
  merge(planes, 2, complexI);
  //4、进行离散傅里叶变化
  dft(complexI, complexI);
  //5、将复数转化为幅值
  split(complexI, planes);//将多通道数组complexI分离成几个单通道数
  //planes[0] = Re(DFT(I));
  //planes[1] = Im(DFT(I));
  //计算矢量幅值
  magnitude(planes[0], planes[1], planes[0]);//将幅值存入planes[0] 
  Mat magnitudeImage = planes[0];
  //6、进行对数尺度缩放
  magnitudeImage += Scalar::all(1);
  log(magnitudeImage, magnitudeImage);//就地操作，求自然对数
  //7、剪切和重分布幅度图像限
  magnitudeImage = magnitudeImage(Rect(0, 0, magnitudeImage.cols & -2, magnitudeImage.rows & -2));//这个&-2什么鬼？？？
  //重新排列傅里叶图像中的象限，使得原点位于图像中心。
  int cx = magnitudeImage.cols / 2;
  int cy = magnitudeImage.rows / 2;
  Mat q0(magnitudeImage, Rect(0, 0, cx, cy)); //ROI区域左上
  Mat q1(magnitudeImage, Rect(cx, 0, cx, cy));//ROI区域右上
  Mat q2(magnitudeImage, Rect(0, cy, cx, cy));//ROI区域左下
  Mat q3(magnitudeImage, Rect(cx, cy, cx, cy));//ROI区域右下
  //交换象限（左上与右下进行交换）
  Mat tmp;
  q0.copyTo(tmp);   //将q0与q3互换
  q3.copyTo(q0);
  tmp.copyTo(q3);
  //交换象限（左下与右上进行交换）
  q1.copyTo(tmp);   //将q1与q2互换
  q2.copyTo(q1);
  tmp.copyTo(q2);
  //8、归一化
  //这一步仍然是为了显示。现在有了重分布后的幅度图，但是幅度值仍然超过了可显示范围【0, 1】。这里使用归一化函数。
  normalize(magnitudeImage, magnitudeImage, 0, 1, NORM_MINMAX);
  //9、显示效果图
  const char *src_str = "原图";
  char dst_gbk[1024] = {0};
  //Utf8ToGbk(src_str, strlen(src_str), dst_gbk, sizeof(dst_gbk));
  imshow("原图", srcImage);
  imshow("频谱幅值", magnitudeImage);
  waitKey(0);
  return 0;
}