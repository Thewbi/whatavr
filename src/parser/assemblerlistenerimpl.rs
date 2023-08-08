use antlr_rust::{tree::{ParseTreeListener, ParseTree}, rule_context::CustomRuleContext};

use super::{
    assemblerlistener::assemblerListener,
    assemblerparser::{assemblerParserContextType, RowContext, Asm_fileContext, ParamContext, MnemonicContext, Asm_intrinsic_usageContext},
};
use crate::{parser::assemblerparser::InstructionContext, assembler::{asm_record::AsmRecord, io_destination::IoDestination}, instructions::instruction_type::InstructionType, ihex_mgmt::ihex_mgmt::Segment, cpu::cpu::{RAMEND, CPU}, LOW, HIGH};
use crate::parser::assemblerparser::Label_definitionContext;
use crate::parser::assemblerparser::ParameterContext;
//use crate::parser::assemblerparser::Macro_usageContext;
//use crate::parser::assemblerparser::Macro_usageContextAttrs;

use std::rc::Rc;

pub struct assemblerListenerImpl {

    pub reg_1: String,
    pub reg_2: String,

    pub data: String,

    pub instruction: String,

    pub last_terminal: String,

    pub mnemonic: String,

    pub label: String,

    pub asm_records: Vec<AsmRecord>,
    pub(crate) asm_encoder: crate::assembler::asm_encoder::AsmEncoder,
    //pub asm_records: Rc<Vec<AsmRecord>>,
    //pub asm_records: Box<Vec<AsmRecord>>,
    //pub asm_records: Vec<Rc<AsmRecord>>

    pub intrinsic_usage: String,

}

impl assemblerListenerImpl {

    // pub fn get_asm_records(&self) -> Vec<AsmRecord> {
    //     self.asm_records.clone()
    // }

    // pub fn get<'a>(&'a mut self, idx: usize) -> &'a mut AsmRecord {
    //     self.asm_records.get_mut(idx).unwrap()
    // }

    // pub fn get_clone(&self) -> AsmRecord {
    //     self.asm_records[0].clone()
    // }

    pub fn reset_self(&mut self) {

        self.reg_1 = String::default();
        self.reg_2 = String::default();

        self.data = String::default();

        self.instruction = String::default();

        self.last_terminal = String::default();

        self.mnemonic = String::default();

        self.label = String::default();

        self.intrinsic_usage = String::default();
    }

}

impl<'input> assemblerListener<'input> for assemblerListenerImpl {

    /**
     * Enter a parse tree produced by {@link assemblerParser#asmFile}.
     * @param ctx the parse tree
     */
    fn enter_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) { 
        log::info!("[enter_asm_file] ..."); 
    }

    /**
     * Exit a parse tree produced by {@link assemblerParser#asmFile}.
     * @param ctx the parse tree
     */
    fn exit_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) {

        // the very last line contained a label definition
        // force a NOP operation and place the label onto that NOP operation
        if self.label != "" {
            let mut asm_record: AsmRecord = AsmRecord::new(self.label.clone(), InstructionType::NOP, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN);
            self.asm_records.push(asm_record);
        }

        // the ihex segment which is filled with source code bytes by the assembler
        let mut assembler_segment: Segment = Segment::new();
        assembler_segment.address = 0u16;
        assembler_segment.size = 0u32;

        // convert the mnemonic instructions into bytes to store into the ihex segment
        //let mut asm_encoder: AsmEncoder = AsmEncoder::new();
        self.asm_encoder.assemble(&mut self.asm_records, &mut assembler_segment);

        // ATmega328p cpu
        let mut cpu: CPU = CPU {
            z: false,
            sph: 0x00u8,
            spl: 0x00u8,
            pc: 0x02i32,
            register_file: [0; 32usize],
            sram: [0; RAMEND as usize],
        };

        log::info!(">>> CPU program execution ...");

        // main loop that executes the instruction
        let mut done: bool = false;
        while !done {

            // get the current instruction
            let temp_pc: i32 = cpu.pc - 0x02;

            // check for end of code
            if assembler_segment.size <= temp_pc as u32 {
                log::info!("End of Code reached! Application Finished!");
                //log_end();
                //return Ok(());

                done = true;
                continue;
            } 

            // execute the next instruction
            cpu.execute_instruction(&assembler_segment);
        }

        log::info!("<<< CPU program execution done.");

    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#row}.
     * @param ctx the parse tree
     */
    fn enter_row(&mut self, _ctx: &RowContext<'input>) {}

    /**
     * Exit a parse tree produced by {@link assemblerParser#row}.
     * @param ctx the parse tree
     */
    fn exit_row(&mut self, _ctx: &RowContext<'input>) {

        log::trace!("[exit_row] ...");

        // do not deal with records that only consists of labels
        if self.label != "" && self.mnemonic == "" {
            return;
        }

        // if self.label == "" && asm_record.instruction_type == InstructionType::UNKNOWN {
        //     // if the .asm file contains a row that just contains an empty label, the label
        //     // is remembered and placed onto the next record. No asm_record is created.
        //     self.reset_self();

        //     return;
        // } 

        let mut asm_record: AsmRecord = AsmRecord::new(String::from(""), InstructionType::UNKNOWN, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN);

        asm_record.instruction_type = InstructionType::from_string(self.mnemonic.as_str());

        if self.reg_1 != "" {

            if self.reg_1.starts_with("r")
            {
                asm_record.reg_1 = self.reg_1[1..].parse::<u16>().unwrap();
            }
        }

        if self.reg_2 != "" {

            if self.reg_2.starts_with("r")
            {
                asm_record.reg_2 = self.reg_2[1..].parse::<u16>().unwrap();
            }
        }

        if self.data != "" {

            asm_record.io_dest = IoDestination::from_string(self.data.as_str());

            if (asm_record.io_dest == IoDestination::UNKNOWN)
            {
                let parse_result = self.data.parse::<u16>();
                if parse_result.is_ok() {
                    asm_record.data = self.data.parse::<u16>().unwrap();
                } else  {
                    asm_record.target_label = self.data.clone();
                }
            }
        }
            
        asm_record.label = self.label.clone();
        self.label = String::default();

        self.asm_records.push(asm_record);

        self.reset_self();

    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#instruction}.
     * @param ctx the parse tree
     */
    fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }

    /**
     * Exit a parse tree produced by {@link assemblerParser#instruction}.
     * @param ctx the parse tree
     */
    fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { 
        log::trace!("[exit_instruction]  {:?}", self.last_terminal); 
    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#label_definition}.
     * @param ctx the parse tree
     */
    fn enter_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) {}

    /**
     * Exit a parse tree produced by {@link assemblerParser#label_definition}.
     * @param ctx the parse tree
     */
    fn exit_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) {
        self.label = self.last_terminal.clone();
        self.last_terminal = String::default();
    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#parameter}.
     * @param ctx the parse tree
     */
    fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) {}

    /**
     * Exit a parse tree produced by {@link assemblerParser#parameter}.
     * @param ctx the parse tree
     */
    fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) {
        log::info!("[exit_parameter] ... ");
    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#param}.
     * @param ctx the parse tree
     */
    fn enter_param(&mut self, _ctx: &ParamContext<'input>) { }

    /**
     * Exit a parse tree produced by {@link assemblerParser#param}.
     * @param ctx the parse tree
     */
    fn exit_param(&mut self, _ctx: &ParamContext<'input>) {

        if self.reg_1 == "" && self.last_terminal.starts_with("r") {
            self.reg_1 = self.last_terminal.clone();
        } else if self.reg_2 == "" && self.last_terminal.starts_with("r") {
            self.reg_2 = self.last_terminal.clone();
        } else {
            self.data = self.last_terminal.clone();
        }

        self.last_terminal = String::default();

        //log::info!("[exit_param] {:?}", self.reg_1);
    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#mnemonic}.
     * @param ctx the parse tree
     */
    fn enter_mnemonic(&mut self, _ctx: &MnemonicContext<'input>) { }

    /**
     * Exit a parse tree produced by {@link assemblerParser#mnemonic}.
     * @param ctx the parse tree
     */
    fn exit_mnemonic(&mut self, _ctx: &MnemonicContext<'input>) { 
        self.mnemonic = self.last_terminal.clone();
        self.last_terminal = String::default();
    }

    /**
     * Enter a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
     * @param ctx the parse tree
     */
    fn enter_asm_intrinsic_usage(&mut self, _ctx: &Asm_intrinsic_usageContext<'input>) { }

    /**
     * Exit a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
     * @param ctx the parse tree
     */
    fn exit_asm_intrinsic_usage(&mut self, _ctx: &Asm_intrinsic_usageContext<'input>) { 
        //log::info!("[exit_asm_intrinsic_usage] {:?}", self.reg_1);

        self.intrinsic_usage = self.last_terminal.clone();

        if "LOW(RAMEND)" == self.intrinsic_usage {
            let low_ramend: u16 = LOW!(RAMEND);
            self.last_terminal = low_ramend.to_string();
        }

        if "HIGH(RAMEND)" == self.intrinsic_usage {
            let high_ramend: u16 = HIGH!(RAMEND);
            self.last_terminal = high_ramend.to_string();
        }

        //self.last_terminal = "10".to_string();
    }

}

impl<'input> ParseTreeListener<'input, assemblerParserContextType> for assemblerListenerImpl {

    fn visit_terminal(
        &mut self,
        _node: &antlr_rust::tree::TerminalNode<'input, assemblerParserContextType>,
    ) {
        let terminal_text = _node.get_text();

        log::trace!("'{}'", terminal_text);

        if terminal_text != ":" && terminal_text != "," && terminal_text != "\r\n" {
            //self.last_terminal = terminal_text;
            self.last_terminal.push_str(terminal_text.as_str());
        }
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'input, assemblerParserContextType>,
    ) {
        //log::info!("visit_error_node()");
    }

    fn enter_every_rule(
        &mut self,
        _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type,
    ) {
        //log::info!("enter_every_rule()");
    }

    fn exit_every_rule(
        &mut self,
        _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type,
    ) {
        //log::info!("exit_every_rule()");
    }
}
