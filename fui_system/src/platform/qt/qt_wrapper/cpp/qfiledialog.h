#ifndef RUST_FUI_QFILEDIALOG_H
#define RUST_FUI_QFILEDIALOG_H

#ifdef __cplusplus
extern "C" {
#endif

void *QFileDialog_getOpenFileName(void *parent,
                              void *caption, void *dir,
                              void *filter, void *selected_filter,
                              int options);

void *QFileDialog_getOpenFileNames(void *parent,
                                   void *caption, void *dir,
                                   void *filter, void *selected_filter,
                                   int options);

void *QFileDialog_getExistingDirectory(void *parent,
                                       void *caption, void *dir,
                                       int options);

void *QFileDialog_getSaveFileName(void *parent,
                                  void *caption, void *dir,
                                  void *filter, void *selected_filter,
                                  int options);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QFILEDIALOG_H
