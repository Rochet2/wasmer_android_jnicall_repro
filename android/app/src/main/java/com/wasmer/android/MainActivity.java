package com.wasmer.android;

import androidx.annotation.Keep;
import androidx.appcompat.app.AppCompatActivity;

import android.content.res.AssetManager;
import android.os.Bundle;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;

public class MainActivity extends AppCompatActivity {
    private static native void JNIExecuteWasm(byte[] module_bytes) throws Exception;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        try {
            // Wait for debugger to attach so
            // we can see all output
            Thread.sleep(1000);
            Thread.sleep(1000);
            Thread.sleep(1000);
            System.out.println("HERE");
            Thread.sleep(1000);
            Thread.sleep(1000);
            Thread.sleep(1000);
            System.out.println("HERE");


            // Load runtime code
            System.loadLibrary("wasmer_android");

            // default code of the empty activity example
            super.onCreate(savedInstanceState);
            setContentView(R.layout.activity_main);

            // Read file into byte array
            AssetManager am = this.getAssets();
            InputStream inputStream = am.open("main.wasm");
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            int reads = inputStream.read();
            while(reads != -1) {
                baos.write(reads);
                reads = inputStream.read();
            }
            byte[] module_bytes = baos.toByteArray();

            // Run the file
            System.out.println("Calling JNIExecuteWasm!");
            JNIExecuteWasm(module_bytes);
            System.out.println("Finished calling JNIExecuteWasm!");
        } catch (Exception e) {
            e.printStackTrace();
        }
        System.out.println("Finished program!");
    }
}
