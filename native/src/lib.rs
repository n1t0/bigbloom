#[macro_use]
extern crate neon;
extern crate bloom;

use neon::prelude::*;
use neon::result::Throw;
use bloom::{BloomFilter, ASMS};

// #[allow(dead_code)] 
pub struct Filter {
    expected_size:       u32,
    false_positive_rate: f32,
    filter:              BloomFilter
}
enum Item {
    Str(String),
    Number(i64),
}

fn extract_item(mut cx: neon::context::CallContext<'_, JsFilter>) 
    -> Result<(Item, neon::context::CallContext<'_, JsFilter>), Throw> {
    let item = cx.argument::<JsValue>(0)?;
    
    if item.is_a::<JsString>() {
        Ok((Item::Str(item.downcast::<JsString>()
            .or_throw(&mut cx)?
            .value()), cx))
    } else if item.is_a::<JsNumber>() {
        Ok((Item::Number(item.downcast::<JsNumber>()
            .or_throw(&mut cx)?
            .value() as i64), cx))
    } else {
        Err(Throw)
    }
}

declare_types! {
    pub class JsFilter for Filter {
        init(mut cx) {
            let size: u32 = cx.argument::<JsNumber>(0)?.value() as u32;
            let rate: f32 = cx.argument::<JsNumber>(1)?.value() as f32;
            let filter = Filter {
                expected_size:       size,
                false_positive_rate: rate,
                filter:              BloomFilter::with_rate(rate as f32, size),
            };
            
            Ok(filter)
        }
        
        method expected_size(mut cx) {
            let this = cx.this();
            
            let size = {
                let guard = cx.lock();
                let size = this.borrow(&guard).expected_size;
                size as f64
            };
            
            Ok(cx.number(size).upcast())
        }
        
        method false_positive_rate(mut cx) {
            let this = cx.this();
            
            let rate = {
                let guard = cx.lock();
                let rate = this.borrow(&guard).false_positive_rate;
                rate as f64
            };
            
            Ok(cx.number(rate).upcast())
        }
        
        method num_bits(mut cx) {
            let this = cx.this();
            
            let bits = {
                let guard = cx.lock();
                let filter = &this.borrow(&guard).filter;
                filter.num_bits() as f64
            };
            
            Ok(cx.number(bits).upcast())
        }
        
        method num_hashes(mut cx) {
            let this = cx.this();
            
            let hashes = {
                let guard = cx.lock();
                let filter = &this.borrow(&guard).filter;
                filter.num_hashes() as f64
            };
            
            Ok(cx.number(hashes).upcast())
        }
        
        method insert(mut cx) {
            let mut this = cx.this();
            
            let (item, mut cx) = extract_item(cx)?;
            
            // Borrow filter
            let result = {
                let guard = cx.lock();
                let mut lock = this.borrow_mut(&guard);
                let mut filter = &mut lock.filter;
                
                match item {
                    Item::Number(n) => { filter.insert(&n) }
                    Item::Str(s)    => { filter.insert(&s) }
                }
            };
            
            Ok(cx.boolean(result).upcast())
        }
        
        method contains(mut cx) {
            let mut this = cx.this();
            
            let (item, mut cx) = extract_item(cx)?;
            
            // Borrow filter
            let result = {
                let guard = cx.lock();
                let mut lock = this.borrow_mut(&guard);
                let filter = &lock.filter;
                
                match item {
                    Item::Number(n) => { filter.contains(&n) }
                    Item::Str(s)    => { filter.contains(&s) }
                }
            };
            
            Ok(cx.boolean(result).upcast())
        }
        
        method clear(mut cx) {
            let mut this = cx.this();
            
            // Borrow filter
            let _ = {
                let guard = cx.lock();
                let mut lock = this.borrow_mut(&guard);
                let mut filter = &mut lock.filter;
                
                filter.clear();
            };
            
            Ok(cx.undefined().upcast())
        }
    }
}


register_module!(mut cx, {
    cx.export_class::<JsFilter>("BloomFilter")
});
