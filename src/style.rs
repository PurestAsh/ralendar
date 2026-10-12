use gpui_kit::{
    self as gpui, Corners, CornersRefinement,
    TextStyleRefinement, rgb,
};
use gpui_kit::{AbsoluteLength, px};
use gpui_kit::{BorderStyle, StyleRefinement};

/*
dracula

Background 	    #282a36 	40 42 54 	231° 15% 18% 	#282a36
Current Line 	#44475a 	68 71 90 	232° 14% 31% 	#44475a
Selection 	    #44475a 	68 71 90 	232° 14% 31% 	#44475a
Foreground 	    #f8f8f2 	248 248 242 	60° 30% 96% 	#f8f8f2
Comment 	    #6272a4 	98 114 164 	225° 27% 51% 	#6272a4
Cyan 	        #8be9fd 	139 233 253 	191° 97% 77% 	#8be9fd
Green 	        #50fa7b 	80 250 123 	135° 94% 65% 	#50fa7b
Orange 	        #ffb86c 	255 184 108 	31° 100% 71% 	#ffb86c
Pink 	        #ff79c6 	255 121 198 	326° 100% 74% 	#ff79c6
Purple 	        #bd93f9 	189 147 249 	265° 89% 78% 	#bd93f9
Red 	        #ff5555 	255 85 85 	0° 100% 67% 	#ff5555
Yellow 	        #f1fa8c 	241 250 140 	65° 92% 76% 	#f1fa8c

*/

//全局div

const BACKGROUND: u32 = 0x282a36;
const CURRENT_LINE: u32 = 0x44475a;
const FOREGROUND: u32 = 0xf8f8f2;

pub fn global_style() -> StyleRefinement {
    StyleRefinement {
        background: Some(rgb(BACKGROUND).into()),
        ..Default::default()
    }
}

pub fn titlebar_style() -> StyleRefinement {
    StyleRefinement {
        text: TextStyleRefinement {
            color: Some(rgb(FOREGROUND).into()),
            ..Default::default()
        },
        background: Some(rgb(BACKGROUND).into()),
        ..Default::default()
    }
}

//侧边栏顶级div
pub fn sidebar() -> StyleRefinement {
    StyleRefinement {
        border_color: Some(rgb(CURRENT_LINE).into()),
        ..Default::default()
    }
}
