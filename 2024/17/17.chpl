use IO;
use IO.FormattedIO;
use Time;
use LinkedLists;

record Registers {
    var a, b, c:int(64);
    var pc: int(8);
}

proc Registers.combo(op: int(8)) {
    if op <= 3 then return op;
    if op == 4 then return a;
    if op == 5 then return b;
    if op == 6 then return c;
    else writeln("deu ruim");
    return 0;
}

proc output(ref outbuf, val) {
    outbuf.push_back(val: int(8));
}

enum opcode {
    i_adv = 0, i_bxl, i_bst, i_jnz, i_bxc, i_out, i_bdv, i_cdv
};

proc read_input() {
    var  reg: Registers;
    var program: LinkedList(opcode);

    readf("Register A: %i\n", reg.a);
    readf("Register B: %i\n", reg.b);
    readf("Register C: %i\n", reg.c);
    readf("\nProgram: ");

    var p = 0;
    var c: uint(8);
    while (true) {
        readf("%i", p);
        program.push_back(p: opcode);
        if !stdin.readByte(c) then break;
        if (c == 10) then break;
    }
    var prog: [0..#program.size] opcode = program;
    return (reg, prog);
}

proc printProgram(const ref program) {
    for i in program.domain {
        if i % 2 == 0 then write(program[i]);
        else writef(" %i, ", program[i]: int);
    }
    writeln();
}

proc printOutput(ref output) {
    for o in output {
        writef("%i,", o);
    }
    writeln();
}

proc compare(const ref program, const ref output) {
    if program.size != output.size then return false;
    for (o, p) in zip(output, program) {
        if p: int != o then return false;
    }
    return true;
}

inline proc exec_adv(ref reg, operand) do reg.a /= (2**reg.combo(operand));
inline proc exec_bxl(ref reg, operand) do reg.b ^= operand;
inline proc exec_bst(ref reg, operand) do reg.b = reg.combo(operand) % 8;
inline proc exec_jnz(ref reg, operand) do if reg.a > 0 then reg.pc = operand - 2; //subtract 2 because of the increment
inline proc exec_bxc(ref reg, operand) do reg.b ^= reg.c;
inline proc exec_out(ref reg, operand, ref outbuf) do output(outbuf, reg.combo(operand) % 8);
inline proc exec_bdv(ref reg, operand) do reg.b = reg.a / (2**reg.combo(operand));
inline proc exec_cdv(ref reg, operand) do reg.c = reg.a / (2**reg.combo(operand));

proc execute(const ref program: [] opcode, ref reg: Registers, ref outbuf) {
    reg.pc = 0;
    while (true) {
        if (reg.pc + 1) >= program.size then break;
        const instruction = program[reg.pc];
        const operand = program[reg.pc + 1];
        select instruction {
            when opcode.i_adv do exec_adv(reg, operand: int(8));
            when opcode.i_bxl do exec_bxl(reg, operand: int(8));
            when opcode.i_bst do exec_bst(reg, operand: int(8));
            when opcode.i_jnz do exec_jnz(reg, operand: int(8));
            when opcode.i_bxc do exec_bxc(reg, operand: int(8));
            when opcode.i_out do exec_out(reg, operand: int(8), outbuf);
            when opcode.i_bdv do exec_bdv(reg, operand: int(8));
            when opcode.i_cdv do exec_cdv(reg, operand: int(8));
            otherwise do break;
        }

        reg.pc += 2;
    }
}

proc main() {
    const (reg, program) = read_input();
    printProgram(program);
    var outbuf: LinkedList(uint(8));
    {
        var reg_temp = reg;
        execute(program, reg_temp, outbuf);
        writeln("Program output: ");
        printOutput(outbuf);
    }

    {
        var i: int(64) = 1;
        var digit = program.size - 1;
        var outarray: [0..#program.size] uint(8) = 255;
        while true {
            var outbuf: LinkedList(uint(8));
            var reg_temp = reg;
            reg_temp.a = i;
            execute(program, reg_temp, outbuf);
            assert(outbuf.size <= outarray.size);

            for (j, o) in zip(0..#outbuf.size, outbuf) do outarray[j] = outbuf.pop_front();
            if compare(program[digit..], outarray[digit..]) {
                digit -= 1;
                if compare(program, outarray) then break;
                else continue;
            }
            i = i + (8 ** digit);
        }

        writeln("Register A with output == program: ", i);
    }
}
