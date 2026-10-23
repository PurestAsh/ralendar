use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    self as gpui, Context, Corners, CornersRefinement,
    TextStyleRefinement, rgb,
};
use gpui_kit::{AbsoluteLength, px};
use gpui_kit::{BorderStyle, StyleRefinement};

use crate::state::State;

/*
dracula

Background 	    #282a36 	40 42 54 	231° 15% 18% 	#282a36
Current Line 	#686B7F 	68 71 90 	232° 14% 31% 	#44475a
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

pub fn global_style(cx: &Context<State>) -> StyleRefinement {
    StyleRefinement {
        background: Some(cx.theme().background.into()),
        ..Default::default()
    }
}

pub fn titlebar_style(cx: &Context<State>) -> StyleRefinement {
    StyleRefinement {
        text: TextStyleRefinement {
            color: Some(cx.theme().foreground),
            ..Default::default()
        },
        background: Some(cx.theme().background.into()),
        ..Default::default()
    }
}

//侧边栏顶级div
pub fn sidebar_style(cx: &Context<State>) -> StyleRefinement {
    StyleRefinement {
        background: Some(cx.theme().background.into()),
        border_color: Some(cx.theme().background),
        ..Default::default()
    }
}
