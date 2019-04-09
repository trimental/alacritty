// Copyright 2016 Joe Wilm, The Alacritty Project Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(any(target_os = "linux", target_os = "bsd"))]
use clipboard::x11_clipboard::{X11ClipboardContext, Clipboard as X11SecondaryClipboard};
use clipboard::{ClipboardProvider, ClipboardContext};

#[derive(Debug)]
pub enum Clipboard {
    Primary,
    Secondary,
}

impl Clipboard {
    pub fn store(&self, text: impl Into<String>) {
        let clipboard = match self {
            #[cfg(any(target_os = "linux", target_os = "bsd"))]
            Clipboard::Secondary => X11ClipboardContext::<X11SecondaryClipboard>::new(),
            #[cfg(not(any(target_os = "linux", target_os = "bsd")))]
            Clipboard::Secondary => return,
            _ => ClipboardProvider::new(),
        };
        println!("STORING IN {:?}", self);

        clipboard
            .and_then(|mut clipboard: ClipboardContext| clipboard.set_contents(text.into()))
            .unwrap_or_else(|err| {
                warn!("Error storing selection to clipboard. {}", err);
            });
    }

    pub fn load(&self) -> Result<String, Box<std::error::Error>> {
        let clipboard = match self {
            #[cfg(any(target_os = "linux", target_os = "bsd"))]
            Clipboard::Secondary => X11ClipboardContext::<X11SecondaryClipboard>::new(),
            _ => ClipboardProvider::new(),
        };
        println!("LOADING FROM {:?}", self);

        clipboard
            .and_then(|mut clipboard: ClipboardContext| clipboard.get_contents())
    }
}
