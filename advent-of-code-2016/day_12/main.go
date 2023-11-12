package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Reg string

type Instruction interface {
	exec(c *CPU)
}

type SourceType int

const (
	SourceTypeRegister SourceType = iota
	SourceTypeLiteral
)

type RegOrLiteral struct {
	typ SourceType
	reg Reg
	lit int32
}

func (r RegOrLiteral) Value(c *CPU) int32 {
	if r.typ == SourceTypeLiteral {
		return r.lit
	}

	return c.RegisterValue(r.reg)
}

type InstructionCpy struct {
	src  RegOrLiteral
	dest Reg
}

func (i InstructionCpy) exec(c *CPU) {
	val := i.src.Value(c)
	c.SetRegisterValue(i.dest, val)
}

type InstructionInc struct {
	register Reg
}

func (i InstructionInc) exec(c *CPU) {
	c.RegisterInc(i.register)
}

type InstructionDec struct {
	register Reg
}

func (i InstructionDec) exec(c *CPU) {
	c.RegisterDec(i.register)
}

type InstructionJnz struct {
	src    RegOrLiteral
	offset int
}

func (i InstructionJnz) exec(c *CPU) {
	val := i.src.Value(c)
	if val == 0 {
		return
	}

	// subtract one here because the first step after execution
	// is to increase the instruction counter.
	c.Jump(i.offset - 1)
}

type CPU struct {
	ip        int
	program   []Instruction
	registers map[Reg]int32
}

func (c *CPU) Run() {
	for c.ip < len(c.program) {
		ins := c.program[c.ip]
		ins.exec(c)
		c.ip++
	}
}

func NewCPU(program []Instruction) *CPU {
	return &CPU{
		ip:        0,
		program:   program,
		registers: map[Reg]int32{"a": 0, "b": 0, "c": 0, "d": 0},
	}
}

func (c *CPU) RegisterValue(reg Reg) int32 {
	return c.registers[reg]
}

func (c *CPU) SetRegisterValue(reg Reg, val int32) {
	c.registers[reg] = val
}

func (c *CPU) RegisterInc(reg Reg) {
	c.registers[reg]++
}

func (c *CPU) RegisterDec(reg Reg) {
	c.registers[reg]--
}

func (c *CPU) Jump(ipOffset int) {
	c.ip += ipOffset
}

func (c *CPU) DumpRegisters() {
	fmt.Printf("%s => %d\n", "a", c.registers["a"])
	fmt.Printf("%s => %d\n", "b", c.registers["b"])
	fmt.Printf("%s => %d\n", "c", c.registers["c"])
	fmt.Printf("%s => %d\n", "d", c.registers["d"])
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	var program []Instruction

	for s.Scan() {
		line := s.Text()
		parts := strings.Split(line, " ")
		ins, err := parseLine(parts)
		if err != nil {
			log.Fatal(err)
		}

		program = append(program, ins)
	}

	fmt.Println("======== part a ==========")
	cpu := NewCPU(program)
	cpu.Run()
	cpu.DumpRegisters()

	fmt.Println("======== part b ==========")
	cpu = NewCPU(program)
	cpu.registers["c"] = 1
	cpu.Run()
	cpu.DumpRegisters()
}

func dumpProgram(program []Instruction) {
	for _, ins := range program {
		printIns(ins)
	}
}

func printIns(ins Instruction) {
	switch ins.(type) {
	case *InstructionDec:
		fmt.Printf("dec: %+v\n", ins)

	case *InstructionInc:
		fmt.Printf("inc: %+v\n", ins)

	case *InstructionCpy:
		fmt.Printf("cpy: %+v\n", ins)

	case *InstructionJnz:
		fmt.Printf("jnz: %+v\n", ins)
	default:
		fmt.Printf("other: %+v\n", ins)
	}
}

type InvalidInstruction struct{}

func (i InvalidInstruction) exec(c *CPU) {}

func parseLine(line []string) (Instruction, error) {
	switch line[0] {
	case "inc":
		return parseInc(line)
	case "dec":
		return parseDec(line)
	case "jnz":
		return parseJnz(line)
	case "cpy":
		return parseCpy(line)
	default:
		return InvalidInstruction{}, fmt.Errorf("invalid instruction %s", line[0])
	}
}

func parseCpy(line []string) (*InstructionCpy, error) {
	src, err := parseRegOrLiteral(line[1])
	if err != nil {
		return nil, err
	}

	dest, err := parseRegister(line[2])
	if err != nil {
		return nil, err
	}

	return &InstructionCpy{
		src:  *src,
		dest: *dest,
	}, nil
}

func parseInc(line []string) (*InstructionInc, error) {
	register, err := parseRegister(line[1])
	if err != nil {
		return nil, err
	}

	return &InstructionInc{register: *register}, nil
}

func parseDec(line []string) (*InstructionDec, error) {
	register, err := parseRegister(line[1])
	if err != nil {
		return nil, err
	}

	return &InstructionDec{register: *register}, nil
}

func parseJnz(line []string) (*InstructionJnz, error) {
	src, err := parseRegOrLiteral(line[1])
	if err != nil {
		return nil, err
	}

	offset, err := strconv.Atoi(line[2])
	if err != nil {
		return nil, err
	}

	return &InstructionJnz{
		src:    *src,
		offset: offset,
	}, nil
}

func parseRegister(input string) (*Reg, error) {
	validRegs := []string{"a", "b", "c", "d"}

	found := false
	for _, v := range validRegs {
		if v == input {
			found = true
			break
		}
	}

	if !found {
		return nil, fmt.Errorf("register %s is invalid", input)
	}

	parsed := Reg(input)
	return &parsed, nil
}

func parseLiteral(input string) (int32, error) {
	v, err := strconv.Atoi(input)
	return int32(v), err
}

func parseRegOrLiteral(input string) (*RegOrLiteral, error) {
	if lit, err := parseLiteral(input); err == nil {
		return &RegOrLiteral{
			typ: SourceTypeLiteral,
			lit: lit,
		}, nil
	}

	if reg, err := parseRegister(input); err == nil {
		return &RegOrLiteral{
			typ: SourceTypeRegister,
			reg: *reg,
		}, nil
	}

	return nil, fmt.Errorf("unable to parse Register or Literal")
}
