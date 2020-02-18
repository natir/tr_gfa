/*
Copyright (c) 2020 Pierre Marijon <pmarijon@mmci.uni-saarland.de>

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

/* crate declaration */
/* cli management*/
#[macro_use]
extern crate structopt;

/* error and logging */
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
extern crate thiserror;

/* other crate */
extern crate csv;
extern crate petgraph;

/* project mod */
mod cli; // Manage cli 
mod gfa; // Serialize Deseralize GFA file
mod algo; // Perform transitive reduction on graph

/* crate use */
use structopt::StructOpt;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    env_logger::init();

    let params = cli::Command::from_args();

    let (unflate, compression) = niffler::get_reader(Box::new(std::fs::File::open(params.input)?))?;

    let input = std::io::BufReader::new(unflate);
    let mut output = std::io::BufWriter::new(niffler::get_writer(Box::new(std::fs::File::create(params.output)?), compression, niffler::compression::Level::One)?);
    
    // Deseralize GFA
    info!("Begin gfa reading and graph building");
    let graph = gfa::deserialize(input, &mut output)?;
    info!("End gfa reading and graph building");

    // Serialize GFA
    info!("Begin graph writing");
    gfa::serialize(output, graph)?;
    info!("End graph writing");   
    Ok(())
}
