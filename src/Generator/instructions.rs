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
}

pub fn subq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tsubq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn mulq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\timulq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn testq(generator: &mut generator::Generator, reg: &str)
{
    generator.output.push_str("\ttestq ");
    generator.output.push_str(reg);
    generator.output.push_str(", ");
    generator.output.push_str(reg);
    generator.output.push_str("\n");
}

pub fn jz(generator: &mut generator::Generator, label: &str)
{
    generator.output.push_str("\tjz ");
    generator.output.push_str(label);
    generator.output.push_str("\n");
}

pub fn jmp(generator: &mut generator::Generator, label: &str)
{
    generator.output.push_str("\tjmp ");
    generator.output.push_str(label);
    generator.output.push_str("\n");
}

pub fn setnz(generator: &mut generator::Generator, reg: &str)
{
    generator.output.push_str("\tsetnz ");
    generator.output.push_str(reg);
    generator.output.push_str("\n");
}

pub fn or(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tor ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn and(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tand ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn movzx(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tmovzx ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmpq(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmpq ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmovg(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmovg ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmovge(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmovge ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmovl(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmovl ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}
pub fn cmovle(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmovle ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmove(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmove ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn cmovne(generator: &mut generator::Generator, src: &str, dst: &str)
{
    generator.output.push_str("\tcmovne ");
    generator.output.push_str(src);
    generator.output.push_str(", ");
    generator.output.push_str(dst);
    generator.output.push_str("\n");
}

pub fn retq(generator: &mut generator::Generator)
{
    generator.output.push_str("\tretq\n");
}

pub fn insert_label(generator: &mut generator::Generator, label: &str)
{
    generator.output.push_str(label);
    generator.output.push_str(":\n");
}