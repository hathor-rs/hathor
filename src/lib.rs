/*!
# Hathor file generator

Allows to quckly generate a set of similar files.

Named after [Hathor](https://en.wikipedia.org/wiki/Hathor) - goddess in ancient Egyptian religion.

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

*This is now in progress project. Things are broken. No warranty provided.*

Currently supported:

- generate set of same size files with predefined content based on string `0123456789`

## Usage

### Installation

```bash
cargo install hathor
```

### Generate files

To generate 5 files with the size 15 in directory `test` use:

```bash
$ mkdir test
$ hathor generate 15 5 test
$ ls test/
0 1 2 3 4
$ cat test/0
012345678901234
```

### Help

Check `--help` for future usage information.

```bash
$ hathor --help
hathor 0.1.0
Hathor - a file generator

USAGE:
    hathor <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generates a set of same size files with repeating pattern `0123456789`
    help        Prints this message or the help of the given subcommand(s)
```
*/

use std::{
    io,
    path::{Path, PathBuf},
};

const DEFAULT_CONTENT: &str = "0123456789";

pub trait FileWriter {
    fn write<P, C>(&mut self, path: P, content: C) -> io::Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>;
}

pub trait FilenameGenerator {
    fn generate(&mut self) -> PathBuf;
}

pub trait PathFilenameGenerator: FilenameGenerator {
    type Name: ToString;
    fn next_name(&mut self) -> Self::Name;

    fn base_path(&self) -> &Path;
}

impl<T, M: ToString> FilenameGenerator for T
where
    T: PathFilenameGenerator<Name = M>,
{
    fn generate(&mut self) -> PathBuf {
        let name = self.next_name().to_string();
        self.base_path().join(name)
    }
}

pub struct OrdinalPathFilenameGenerator {
    root: PathBuf,
    current: usize,
}

impl<T> From<T> for OrdinalPathFilenameGenerator
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        Self {
            root: value.as_ref().into(),
            current: 0,
        }
    }
}

impl PathFilenameGenerator for OrdinalPathFilenameGenerator {
    type Name = usize;

    fn next_name(&mut self) -> usize {
        let current = self.current;
        self.current += 1;
        current
    }

    fn base_path(&self) -> &Path {
        &self.root
    }
}

pub struct DefaultFileWriter {}

impl FileWriter for DefaultFileWriter {
    fn write<P, C>(&mut self, path: P, content: C) -> io::Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        std::fs::write(path, content)
    }
}

pub struct FileGeneratorBuilder {}

impl FileGeneratorBuilder {
    pub fn with_size(size: usize) -> SizedFileGenerator {
        SizedFileGenerator { size }
    }
}

pub trait FileGenerator: Sized {
    fn write<PG, F>(&self, path_generator: &mut PG, file_writer: &mut F) -> io::Result<()>
    where
        PG: FilenameGenerator,
        F: FileWriter;

    fn generate_to<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.write(
            &mut OrdinalPathFilenameGenerator::from(path),
            &mut DefaultFileWriter {},
        )
    }

    fn repeat(self, count: usize) -> RepeatFileGenerator<Self> {
        RepeatFileGenerator {
            count,
            generator: self,
        }
    }
}

pub struct SizedFileGenerator {
    size: usize,
}

impl FileGenerator for SizedFileGenerator {
    fn write<PG, F>(&self, path_generator: &mut PG, file_writer: &mut F) -> io::Result<()>
    where
        PG: FilenameGenerator,
        F: FileWriter,
    {
        let buffer: Vec<u8> = DEFAULT_CONTENT.bytes().cycle().take(self.size).collect();

        file_writer.write(path_generator.generate(), buffer)
    }
}

pub struct RepeatFileGenerator<G: FileGenerator> {
    count: usize,
    generator: G,
}

impl<G: FileGenerator> FileGenerator for RepeatFileGenerator<G> {
    fn write<PG, F>(&self, path_generator: &mut PG, file_writer: &mut F) -> io::Result<()>
    where
        PG: FilenameGenerator,
        F: FileWriter,
    {
        for _ in 0..self.count {
            self.generator.write(path_generator, file_writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{FileGenerator, FileGeneratorBuilder, FileWriter, OrdinalPathFilenameGenerator};

    struct TestFileGenerator {
        files: Vec<(PathBuf, Vec<u8>)>,
    }

    impl FileWriter for TestFileGenerator {
        fn write<P, C>(&mut self, path: P, content: C) -> std::io::Result<()>
        where
            P: AsRef<std::path::Path>,
            C: AsRef<[u8]>,
        {
            self.files
                .push((path.as_ref().into(), content.as_ref().into()));

            Ok(())
        }
    }

    #[test]
    fn repeated_file_generator() {
        let mut test_file_generator = TestFileGenerator { files: vec![] };

        let _ = FileGeneratorBuilder::with_size(1)
            .repeat(3)
            .write(
                &mut OrdinalPathFilenameGenerator::from("/test/path"),
                &mut test_file_generator,
            )
            .unwrap();

        assert_eq!(
            test_file_generator.files,
            vec![
                (PathBuf::from("/test/path/0"), b"0".to_vec()),
                (PathBuf::from("/test/path/1"), b"0".to_vec()),
                (PathBuf::from("/test/path/2"), b"0".to_vec())
            ]
        );
    }
}
