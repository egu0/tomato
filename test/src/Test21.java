public class Test21 {
    public int test(){
        int a =  0;
        int[] arr = new int[2];
        for(int i = 0; i < arr.length ; i++){
            arr[i] = a ++;
        }
        return arr[1] == 2 ? 20240325 : 20240324 ;
    }

}
