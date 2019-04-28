#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include "pyxelcore/constants.h"
#include "pyxelcore/window_info.h"

namespace pyxelcore {

class Resource;
class Input;
class Graphics;
class Audio;

class System {
 public:
  System(int32_t width,
         int32_t height,
         const char* caption = DEFAULT_CAPTION,
         int32_t scale = DEFAULT_SCALE,
         const int32_t* palette_color = DEFAULT_PALETTE,
         int32_t fps = DEFAULT_FPS,
         int32_t border_width = DEFAULT_BORDER_WIDTH,
         int32_t border_color = DEFAULT_BORDER_COLOR);
  ~System();

  Resource* Resource() const { return resource_; }
  Input* Input() const { return input_; }
  Graphics* Graphics() const { return graphics_; }
  Audio* Audio() const { return audio_; }
  const int32_t* PaletteColor() const { return palette_color_; }

  int32_t Width() const { return window_info_.screen_width; }
  int32_t Height() const { return window_info_.screen_height; }
  int32_t FrameCount() const { return frame_count_; }

  void Run(void (*update)(), void (*draw)());

  static int32_t GetConstantNumber(const char* name);
  static const char* GetConstantString(const char* name);
  static void RaiseError(const char* message);

 private:
  pyxelcore::Input* input_;
  pyxelcore::Resource* resource_;
  pyxelcore::Graphics* graphics_;
  pyxelcore::Audio* audio_;

  int32_t fps_;
  int32_t border_width_;
  int32_t border_color_;
  int32_t frame_count_;
  struct WindowInfo window_info_;
  int32_t palette_color_[COLOR_COUNT];

  void SetupWindow(const char* caption,
                   int32_t width,
                   int32_t height,
                   int32_t scale);
  void RenderWindow();
  void UpdateScreenTexture();
  void UpdateWindowInfo();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
