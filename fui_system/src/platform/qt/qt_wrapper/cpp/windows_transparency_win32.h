enum ACCENT_STATE : INT {				// Affects the rendering of the background of a window.
    ACCENT_DISABLED = 0,					// Default value. Background is black.
    ACCENT_ENABLE_GRADIENT = 1,				// Background is GradientColor, alpha channel ignored.
    ACCENT_ENABLE_TRANSPARENTGRADIENT = 2,	// Background is GradientColor.
    ACCENT_ENABLE_BLURBEHIND = 3,			// Background is GradientColor, with blur effect.
    ACCENT_ENABLE_ACRYLICBLURBEHIND = 4,	// Background is GradientColor, with acrylic blur effect.
    ACCENT_ENABLE_HOSTBACKDROP = 5,			// Unknown.
    ACCENT_INVALID_STATE = 6				// Unknown. Seems to draw background fully transparent.
};

struct ACCENT_POLICY {			// Determines how a window's background is rendered.
    ACCENT_STATE	AccentState;	// Background effect.
    UINT			AccentFlags;	// Flags. Set to 2 to tell GradientColor is used, rest is unknown.
    COLORREF		GradientColor;	// Background color.
    LONG			AnimationId;	// Unknown
};

enum WINDOWCOMPOSITIONATTRIB : INT {	// Determines what attribute is being manipulated.
    WCA_ACCENT_POLICY = 0x13				// The attribute being get or set is an accent policy.
};

struct WINDOWCOMPOSITIONATTRIBDATA {	// Options for [Get/Set]WindowCompositionAttribute.
    WINDOWCOMPOSITIONATTRIB	Attrib;			// Type of what is being get or set.
    LPVOID					pvData;			// Pointer to memory that will receive what is get or that contains what will be set.
    UINT					cbData;			// Size of the data being pointed to by pvData.
};

typedef BOOL (WINAPI* PFN_SET_WINDOW_COMPOSITION_ATTRIBUTE)(HWND, const WINDOWCOMPOSITIONATTRIBDATA *);

void EnableBlurWin32(HWND hwnd, bool enabled) {
    ACCENT_POLICY policy = {
            enabled ? ACCENT_ENABLE_BLURBEHIND : ACCENT_DISABLED,
            0,
            0,
            0
    };

    const WINDOWCOMPOSITIONATTRIBDATA data = {
            WCA_ACCENT_POLICY,
            &policy,
            sizeof(policy)
    };

    static const auto SetWindowCompositionAttribute =
            reinterpret_cast<PFN_SET_WINDOW_COMPOSITION_ATTRIBUTE>(GetProcAddress(GetModuleHandle(L"user32.dll"), "SetWindowCompositionAttribute"));

    SetWindowCompositionAttribute(hwnd, &data);
}
