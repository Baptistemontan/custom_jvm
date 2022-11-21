class HelloWorld
{
    String test = null;

    static long i = 0;

    static double f = 90.45;

    public static int add(int a, int b) {
        return a + b;
    }

    public static int fib(int n) {
        int a = 0;
        int b = 1;
        for(int i = 0; i < n; i++) {
            int tmp = b + a;
            a = b;
            b = tmp;
        }
        return a;
    }

    public static void main(String[] args)
    {
    	System.out.println("Hello, World.");

        if (f == 56.34) {
            System.out.println("Test");
        }
    }
}
