package JNI;


public class ExcelJNI{
	 static {
        System.loadLibrary("rust_java_demo");
    }

    public static void main(String[] args) {
        init();
    }

    static native void init();

}
