#![no_std]

extern crate bobbin_common;

pub trait ReadWrite {
  fn read(&self, rw: u8) -> u8;
  fn write(&self, rw: u8, val: u8);
}

pub trait TryReadWrite {
  type Error;
  fn try_read(&self, rw: u8) -> Result<u8, Self::Error>;
  fn try_write(&self, rw: u8, val: u8) -> Result<(), Self::Error>;
}

pub struct Rfm95 {}
