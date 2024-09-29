#include "ImagePipeline.hpp"

void ImagePipeline::appendStep(std::function<cv::Mat&(cv::Mat&)> step)
{
	steps.push_back(step);
}

cv::Mat& ImagePipeline::process(cv::Mat & img)
{
	stepImg.clear();

	for (auto &step : steps) {
		stepImg.push_back( step(img).clone() );
	}

	return img;
}



cv::Mat ImagePipeline::getStepImage(int step)
{
	return stepImg.at(step);
}

void ImagePipeline::popStep() {
	steps.pop_back();
}