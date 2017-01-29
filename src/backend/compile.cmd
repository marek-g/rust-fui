@echo off
set FXC="%DXSDK_DIR%\Utilities\bin\x64\fxc.exe" -nologo
if not exist data mkdir data
%FXC% /T vs_4_0 /E Vertex /Fo data/colored_vertex.fx shader/colored.hlsl
%FXC% /T ps_4_0 /E Pixel /Fo data/colored_pixel.fx shader/colored.hlsl
%FXC% /T vs_4_0 /E Vertex /Fo data/textured_vertex.fx shader/textured.hlsl
%FXC% /T ps_4_0 /E Pixel /Fo data/textured_pixel.fx shader/textured.hlsl
