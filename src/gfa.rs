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

/* local use */
use crate::algo;

/* crate use */
use petgraph;
use anyhow::{Context, Result};

pub type Gfa = petgraph::graph::DiGraph<(String, char), String>;

type Name2Id = std::collections::HashMap<(String, char), petgraph::graph::NodeIndex>;

fn get_node(graph: &mut Gfa, name2id: &mut Name2Id, weight: (String, char)) -> petgraph::graph::NodeIndex {
    if let Some(id) = name2id.get(&weight) {
	*id
    } else {
	let id = graph.add_node(weight.clone());
	name2id.insert(weight, id);
	id
    }
}

fn add_edge(graph: &mut Gfa, name2id: &mut Name2Id, source: (String, char), target: (String, char), length: &String) {

    let node1 = get_node(graph, name2id, source);
    let node2 = get_node(graph, name2id, target);

    let node1_succ = algo::successors(graph, node1);
    let node2_succ = algo::successors(graph, node2);
    
    // node1 -> x node2 -> y
    let xz_yz: algo::NodeSet = node1_succ.intersection(&node2_succ).copied().collect();
    if xz_yz.len() != 0 {
	for z in xz_yz {
	    if let Some(edge) = graph.find_edge(node1, z) {
		graph.remove_edge(edge);
	    }
	}
	
	graph.add_edge(node1, node2, length.to_string());

	return
    }

    let node1_pred = algo::predecessors(graph, node1);
    let node2_pred = algo::predecessors(graph, node2);
    // node1 -> y node2 -> z
    let xy_xz: algo::NodeSet = node1_pred.intersection(&node2_pred).copied().collect();
    if xy_xz.len() != 0 {
	for x in xy_xz {
	    if let Some(edge) = graph.find_edge(x, node2) {
		graph.remove_edge(edge);
	    }
	}

	graph.add_edge(node1, node2, length.to_string());

	return
    }
	
    // node1 -> x node2 -> z
    let xy_yz: algo::NodeSet = node1_succ.intersection(&node2_pred).copied().collect();
    if xy_yz.len() != 0 {
	return
    } else {
	graph.add_edge(node1, node2, length.to_string());
    }
}

fn rev_ori(a: char) -> char {
    match a {
	'+' => '-',
	'-' => '+',
	_ => unreachable!()
    }
}

pub fn deserialize<R, W>(input: R, output: W) -> Result<Gfa>
where R: std::io::Read,
      W: std::io::Write,
{
    let mut reader = csv::ReaderBuilder::new()
	.delimiter(b'\t')
	.flexible(true)
	.has_headers(false)
	.from_reader(input);

    let mut writer = csv::WriterBuilder::new()
	.delimiter(b'\t')
	.flexible(true)
	.from_writer(output);
    
    let mut name2id = std::collections::HashMap::new();
    
    let mut graph = Gfa::new();
    
    let mut record = csv::StringRecord::new();
    while reader.read_record(&mut record)? {
	if &record[0] == "L" {
	    let id1 = record[1].to_string();
	    let ori1 = record[2].chars().next().unwrap();
	    let id2 = record[3].to_string();
	    let ori2 = record[4].chars().next().unwrap();

	    let edge = record[5].to_string();

	    add_edge(&mut graph, &mut name2id, (id1.clone(), ori1), (id2.clone(), ori2), &edge);
	    add_edge(&mut graph, &mut name2id, (id2, rev_ori(ori2)), (id1, rev_ori(ori1)), &edge);
	} else {
	    writer.write_record(&record)?;
	}
    }
    
    Ok(graph)
}

pub fn serialize<W>(output: W, graph: Gfa) -> Result<()>
where W: std::io::Write,
{
    let mut writer = csv::WriterBuilder::new()
	.delimiter(b'\t')
	.flexible(true)
	.from_writer(output);

    /* write link */
    let mut rev_write = std::collections::HashSet::new();
    for edge in graph.edge_indices() {
	if let Some((source, target)) = graph.edge_endpoints(edge) {
	    if let Some(overlap) = graph.edge_weight(edge) {
		if let Some((src_id, src_ori)) = graph.node_weight(source) {
		    if let Some((tgt_id, tgt_ori)) = graph.node_weight(target) {
			if rev_write.contains(&(src_id, *src_ori, tgt_id, *tgt_ori)) {
			    continue
			}
			
			writer.write_record(&["L", src_id, &src_ori.to_string(), tgt_id, &tgt_ori.to_string(), overlap])?;
			rev_write.insert((tgt_id, rev_ori(*tgt_ori), src_id, rev_ori(*src_ori)));
		    }
		}
	    }
	}
    }

    Ok(())
}
