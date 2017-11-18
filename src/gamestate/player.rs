use super::Frame;
use super::Throw;
use failure::Error;

#[derive(Debug, Fail)]
enum FrameError {
    #[fail(display = "max frame amount reached")]
    MaxedFrames,
}

#[derive(Debug, Fail)]
enum ThrowError {
    #[fail(display = "max throws for frame reached")]
    MaxThrows,
    #[fail(display = "unexpected strike")]
    UnexpectedStrike,
}

pub struct Player {
    name: String,
    frames: Vec<Frame>,
}

impl Player {
    pub fn new<S: Into<String>>(name: S) -> Player {
        Player {
            name: name.into(),
            frames: Vec::with_capacity(10),
        }
    }

    pub fn add_frame(&mut self) -> Result<(), Error> {
        if self.frames.len() >= 10 {
            Err(FrameError::MaxedFrames.into())
        } else {
            self.frames.push(Frame::default());
            Ok(())
        }
    }

    pub fn add_throw(&mut self, throw: Throw) {
        let last_frame = self.frames.last_mut();
        match last_frame {
            Some(frame) => frame.add_throw(throw),
            None => unimplemented!(),
        }
    }
}
