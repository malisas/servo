/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::DogeBinding::{DogeMethods, DogeInit, Wrap as DogeWrap};
use dom::bindings::error::{Error, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::str::ByteString;
use rand;
use rand::Rng;

#[dom_struct]
pub struct Doge {
    reflector_: Reflector,
    such_list: DOMRefCell<Vec<ByteString>>,
}

impl Doge {
    pub fn new_inherited() -> Doge {
        Doge {
            reflector_: Reflector::new(),
            such_list: DOMRefCell::new(vec![]),
        }
    }

    pub fn new(global: GlobalRef) -> Root<Doge> {
        reflect_dom_object(box Doge::new_inherited(), global, DogeWrap)
    }

    // https://jeenalee.github.io/doge-standard/#dom-doge
    pub fn Constructor(global: GlobalRef, init: Option<DogeInit>) -> Fallible<Root<Doge>> {
        // Step 1
        let new_doge = Doge::new(global);
        // Step 2
        if let Some(very_init) = init {
            for word in very_init {
                new_doge.Append(word);
            }
        }
        // Step 3
        Ok(new_doge)
    }
}

impl DogeMethods for Doge {
    // https://jeenalee.github.io/doge-standard/#dom-doge-append
    fn Append(&self, word: ByteString) -> () {
        *&self.such_list.borrow_mut().push(word);
    }

    // https://jeenalee.github.io/doge-standard/#dom-doge-random
    fn Random(&self) -> Fallible<ByteString> {
        // Step 1
        let l = self.such_list.borrow();
        // Step 2
        if l.len() == 0 {
            return Err(Error::Type("Such list is empty".to_string()));
        } else {
            // Step 3
            let wow_number = rand::thread_rng().gen_range(0, l.len());
            return Ok(l[wow_number].clone());
        }
    }
}
