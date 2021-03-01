package term;

import java.util.List;

import com.google.gson.annotations.SerializedName;

import symbol.ExternSymbol;

public class Program {

    @SerializedName("subs")
    private List<Term<Sub>> subs;
    @SerializedName("extern_symbols")
    private List<ExternSymbol> externSymbols;
    @SerializedName("entry_points")
    private List<Tid> entryPoints;
    @SerializedName("image_base")
    private String imageBase;

    public Program() {
    }

    public Program(List<Term<Sub>> subs) {
        this.setSubs(subs);
    }

    public Program(List<Term<Sub>> subs, List<Tid> entryPoints, String imageBase) {
        this.setSubs(subs);
        this.setEntryPoints(entryPoints);
        this.setImageBase(imageBase);
    }


    public List<Term<Sub>> getSubs() {
        return subs;
    }

    public void setSubs(List<Term<Sub>> subs) {
        this.subs = subs;
    }

    public void addSub(Term<Sub> sub) {
        this.subs.add(sub);
    }

    public List<ExternSymbol> getExternSymbols() {
        return externSymbols;
    }

    public void setExternSymbols(List<ExternSymbol> extern_symbols) {
        this.externSymbols = extern_symbols;
    }

    public List<Tid> getEntryPoints() {
        return entryPoints;
    }

    public void setEntryPoints(List<Tid> entryPoints) {
        this.entryPoints = entryPoints;
    }

    public String getImageBase() {
        return imageBase;
    }

    public void setImageBase(String imageBase) {
        this.imageBase = imageBase;
    }
}
