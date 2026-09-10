#![no_main]

//! tuack-ng 示例插件。

use std::path::PathBuf;

use tuack_plugin_sdk::Document;
use tuack_plugin_sdk::log::info;
use tuack_plugin_sdk::{
    DumpDocument, Dumper, Error, OutputFile, Processor, ProcessorOutput, RenderDocument, Renderer,
    dumper, processor, renderer,
};

/// 处理器示例。
struct ExampleProcessor;

impl Processor for ExampleProcessor {
    fn new() -> Self {
        ExampleProcessor
    }

    fn process(&self, doc: Document) -> Result<ProcessorOutput, Error> {
        info!("hello world");
        Ok(ProcessorOutput {
            ast: doc,
            warnings: Vec::new(),
        })
    }
}

/// 渲染器示例。
struct ExampleRenderer;

impl Renderer for ExampleRenderer {
    fn new() -> Self {
        ExampleRenderer
    }

    fn render(&self, _doc: RenderDocument) -> Result<(PathBuf, Vec<OutputFile>), Error> {
        info!("hello world");
        Ok((PathBuf::new(), Vec::new()))
    }
}

/// 导出器示例。
struct ExampleDumper;

impl Dumper for ExampleDumper {
    fn new() -> Self {
        ExampleDumper
    }

    fn dump(&self, _doc: DumpDocument) -> Result<(Vec<OutputFile>, Vec<String>), Error> {
        info!("hello world");
        Ok((Vec::new(), Vec::new()))
    }
}

processor!(ExampleProcessor, process_example);
renderer!(ExampleRenderer, render_example);
dumper!(ExampleDumper, dump_example);
