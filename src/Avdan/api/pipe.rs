/*
 * pub mod Pipe {
 *     use std::io::{Read, Write};
 *
 *     pub trait Input {
 *         fn process(&self, source: &impl Output, bytes: &[u8]) -> ();
 *     }
 *
 *     pub trait Output {
 *         fn _pipe<I>(&self, dest : I, bytes: &[u8] ) -> ()
 *             where I : Input {
 *             dest.process(self, bytes);
 *         }
 *     }
 * }
*/
