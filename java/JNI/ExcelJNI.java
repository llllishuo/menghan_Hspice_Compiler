package JNI;


public class ExcelJNI{
	 static {
        System.loadLibrary("HspiceCompiler");
    }

    public static void main(String[] args) {
        init();
    }

    static native void init();

}
