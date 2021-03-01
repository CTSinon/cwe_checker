package term;

import bil.RegisterProperties;
import bil.Variable;
import internal.RegisterConvention;

import java.util.List;

import com.google.gson.annotations.SerializedName;

public class Project {
    @SerializedName("program")
    private Term<Program> program;
    @SerializedName("stack_pointer_register")
    private Variable stackPointerRegister;
    @SerializedName("register_properties")
    private List<RegisterProperties> registerProperties;
    @SerializedName("cpu_architecture")
    private String cpuArch;
    @SerializedName("register_calling_convention")
    private List<RegisterConvention> conventions;

    public Project() {
    }

    public Project(Term<Program> program, String cpuArch, Variable stackPointerRegister, List<RegisterConvention> conventions) {
        this.setProgram(program);
        this.setCpuArch(cpuArch);
        this.setStackPointerRegister(stackPointerRegister);
        this.setRegisterConvention(conventions);
    }

    public Term<Program> getProgram() {
        return program;
    }

    public void setProgram(Term<Program> program) {
        this.program = program;
    }

    public Variable getStackPointerRegister() {
        return stackPointerRegister;
    }

    public void setStackPointerRegister(Variable stackPointerRegister) {
        this.stackPointerRegister = stackPointerRegister;
    }

    public String getCpuArch() {
        return cpuArch;
    }

    public void setCpuArch(String cpuArch) {
        this.cpuArch = cpuArch;
    }

    public List<RegisterConvention> getRegisterConvention() {
        return conventions;
    }

    public void setRegisterConvention(List<RegisterConvention> conventions) {
        this.conventions = conventions;
    }

    public List<RegisterProperties> getRegisterProperties() {
        return registerProperties;
    }

    public void setRegisterProperties(List<RegisterProperties> registerProperties) {
        this.registerProperties = registerProperties;
    }
}
