#include "qsurfaceformat.h"
#include <QSurfaceFormat>

void QSurfaceFormat_setDefault(int stencilBufferSize)
{
    QSurfaceFormat format;
    format.setStencilBufferSize(stencilBufferSize);
    QSurfaceFormat::setDefaultFormat(format);
}
