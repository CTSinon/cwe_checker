package term;

import java.util.ArrayList;
import java.util.List;

import com.google.gson.annotations.SerializedName;

public class Blk {
    @SerializedName("defs")
    private List<Term<Def>> defs;
    @SerializedName("jmps")
    private List<Term<Jmp>> jmps;

    public Blk() {
        this.setDefs(new ArrayList<>());
        this.setJmps(new ArrayList<>());
    }

    public Blk(List<Term<Def>> defs, List<Term<Jmp>> jmps) {
        this.setDefs(defs);
        this.setJmps(jmps);
    }

    public List<Term<Def>> getDefs() {
        return defs;
    }

    public void setDefs(List<Term<Def>> defs) {
        this.defs = defs;
    }

    public List<Term<Jmp>> getJmps() {
        return jmps;
    }

    public void setJmps(List<Term<Jmp>> jmps) {
        this.jmps = jmps;
    }

    public void addDef(Term<Def> def) {
        this.defs.add(def);
    }

    public void addJmp(Term<Jmp> jmp) {
        this.jmps.add(jmp);
    }

    public void addMultipleDefs(List<Term<Def>> defs) {
        this.defs.addAll(defs);
    }

    public void addMultipleJumps(List<Term<Jmp>> jmps) {
        this.jmps.addAll(jmps);
    }


}
