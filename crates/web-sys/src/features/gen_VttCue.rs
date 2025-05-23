#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = TextTrackCue , extends = EventTarget , extends = :: js_sys :: Object , js_name = VTTCue , typescript_type = "VTTCue")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VttCue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub type VttCue;
    #[cfg(feature = "VttRegion")]
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = region)]
    #[doc = "Getter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    pub fn region(this: &VttCue) -> Option<VttRegion>;
    #[cfg(feature = "VttRegion")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = region)]
    #[doc = "Setter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    pub fn set_region(this: &VttCue, value: Option<&VttRegion>);
    #[cfg(feature = "DirectionSetting")]
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = vertical)]
    #[doc = "Getter for the `vertical` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    pub fn vertical(this: &VttCue) -> DirectionSetting;
    #[cfg(feature = "DirectionSetting")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = vertical)]
    #[doc = "Setter for the `vertical` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    pub fn set_vertical(this: &VttCue, value: DirectionSetting);
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = snapToLines)]
    #[doc = "Getter for the `snapToLines` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn snap_to_lines(this: &VttCue) -> bool;
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = snapToLines)]
    #[doc = "Setter for the `snapToLines` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_snap_to_lines(this: &VttCue, value: bool);
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = line)]
    #[doc = "Getter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn line(this: &VttCue) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = line)]
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    #[deprecated]
    pub fn set_line(this: &VttCue, value: &::wasm_bindgen::JsValue);
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = line)]
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_line_f64(this: &VttCue, value: f64);
    #[cfg(feature = "AutoKeyword")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = line)]
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutoKeyword`, `VttCue`*"]
    pub fn set_line_auto_keyword(this: &VttCue, value: AutoKeyword);
    #[cfg(feature = "LineAlignSetting")]
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = lineAlign)]
    #[doc = "Getter for the `lineAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    pub fn line_align(this: &VttCue) -> LineAlignSetting;
    #[cfg(feature = "LineAlignSetting")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = lineAlign)]
    #[doc = "Setter for the `lineAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    pub fn set_line_align(this: &VttCue, value: LineAlignSetting);
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = position)]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn position(this: &VttCue) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = position)]
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    #[deprecated]
    pub fn set_position(this: &VttCue, value: &::wasm_bindgen::JsValue);
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = position)]
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_position_f64(this: &VttCue, value: f64);
    #[cfg(feature = "AutoKeyword")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = position)]
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutoKeyword`, `VttCue`*"]
    pub fn set_position_auto_keyword(this: &VttCue, value: AutoKeyword);
    #[cfg(feature = "PositionAlignSetting")]
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = positionAlign)]
    #[doc = "Getter for the `positionAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    pub fn position_align(this: &VttCue) -> PositionAlignSetting;
    #[cfg(feature = "PositionAlignSetting")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = positionAlign)]
    #[doc = "Setter for the `positionAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    pub fn set_position_align(this: &VttCue, value: PositionAlignSetting);
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn size(this: &VttCue) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = size)]
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_size(this: &VttCue, value: f64);
    #[cfg(feature = "AlignSetting")]
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = align)]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    pub fn align(this: &VttCue) -> AlignSetting;
    #[cfg(feature = "AlignSetting")]
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = align)]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    pub fn set_align(this: &VttCue, value: AlignSetting);
    # [wasm_bindgen (structural , method , getter , js_class = "VTTCue" , js_name = text)]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn text(this: &VttCue) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "VTTCue" , js_name = text)]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_text(this: &VttCue, value: &str);
    #[wasm_bindgen(catch, constructor, js_class = "VTTCue")]
    #[doc = "The `new VttCue(..)` constructor, creating a new instance of `VttCue`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/VTTCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn new(start_time: f64, end_time: f64, text: &str) -> Result<VttCue, JsValue>;
    #[cfg(feature = "DocumentFragment")]
    # [wasm_bindgen (method , structural , js_class = "VTTCue" , js_name = getCueAsHTML)]
    #[doc = "The `getCueAsHTML()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `VttCue`*"]
    pub fn get_cue_as_html(this: &VttCue) -> DocumentFragment;
}
