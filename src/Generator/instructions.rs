use crate::generator;


// convention: all values are returned on the stack
pub fn push(generator: &mut generator::Generator, reg: &str)
{
    generator.output.push_str("\tpushq ");
    generator.output.push_str(reg);
    generator.output.push_str("\n");
    generator.stack_size += 1;
}

pub fn pop(generator: &mut generator::Generator, reg: &str)
{
    generator.output.push_str("\tpopq ");
    generator.output.push_str(reg);
    generator.output.push_str("\n");
    generator.stack_size -= 1;
}

pub fn movq_immediate(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tmovq $");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn movq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tmovq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}


pub fn addq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\taddq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
    push(generator, dst);
}

pub fn mulq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\timulq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
    push(generator, dst);
}