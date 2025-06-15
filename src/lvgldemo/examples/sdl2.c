#include <stdio.h>
#include <stdlib.h>
#include <SDL.h>
//#include <synchapi.h>

int main(int argc, char* argv[]) {
    SDL_Init( SDL_INIT_EVERYTHING );    // 初始化SDL2所有部分

    SDL_Window* win = SDL_CreateWindow( "my window", 100, 100, 640, 480, SDL_WINDOW_SHOWN );    // 创建窗口
    SDL_Surface* winSurface = SDL_GetWindowSurface( win );   // 基于窗口创建“表面”
    SDL_UpdateWindowSurface( win );                          // 更新窗口
    SDL_FillRect( winSurface, NULL, SDL_MapRGB( winSurface->format, 255, 90, 120 ));            // 绘制矩形
    SDL_UpdateWindowSurface( win ); // 更新窗口
    while (1);                              // 卡住界面(保持窗口，)
    SDL_DestroyWindow( win );       // 销毁窗口
    win = NULL; winSurface = NULL;         // 释放变量
    SDL_Quit();                            // 关闭SDL2

    return 0;
}
