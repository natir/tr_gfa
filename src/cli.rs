/*
Copyright (c) 2020 Pierre Marijon <pmarijon@mpi-inf.mpg.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* crate use */
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.1",
    author = "Pierre Marijon <pmarijon@mpi-inf.mpg.de>",
    name = "trGFA",
    about = "Use myers algorithm to remove transitive edge in gfa file"
)]
pub struct Command {
    #[structopt(
	short = "i",
	long = "input",
	required = true,
	help = "path of gfa input (gzip, bzip2, lzma compression allowed)"
    )]
    pub input: String,

    #[structopt(
	short = "o",
	long = "output",
	required = true,
	help = "path of gfa output (write with same compression as input)"
    )]
    pub output: String,
}
