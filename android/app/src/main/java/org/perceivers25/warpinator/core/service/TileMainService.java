package org.perceivers25.warpinator.core.service;

import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Build;
import android.os.IBinder;
import android.service.quicksettings.Tile;
import android.service.quicksettings.TileService;
import android.util.Log;
import android.widget.Toast;

import androidx.annotation.RequiresApi;

import org.perceivers25.warpinator.core.utils.Utils;

import dagger.hilt.android.AndroidEntryPoint;

@RequiresApi(api = Build.VERSION_CODES.N)
@AndroidEntryPoint
public class TileMainService extends TileService {
    private Intent serviceIntent;
    private ServiceConnection serviceConnection;
    private MainServiceBinder binder;

    public static void requestListeningState(Context context) {
        requestListeningState(context, new ComponentName(context, TileMainService.class));
    }

    @Override
    public void onCreate() {
        super.onCreate();
        serviceIntent = new Intent(this, MainService.class);
    }

    @Override
    public void onStartListening() {
        super.onStartListening();
        boolean isRunning = Utils.INSTANCE.isMyServiceRunning(this, MainService.class);
        updateTile(isRunning);
        if (isRunning && binder == null) {
            joinService();
        }
    }

    @Override
    public void onDestroy() {
        if (serviceConnection != null) {
            unbindService(serviceConnection);
            serviceConnection = null;
        }
//        if (unregisterObserver != null) {
//            unregisterObserver.dispose();
//            unregisterObserver = null;
//        }
        binder = null;
        super.onDestroy();
    }

    @Override
    public void onClick() {
        super.onClick();
        switch (getQsTile().getState()) {
            case Tile.STATE_ACTIVE:
                stopService();
                break;
            case Tile.STATE_INACTIVE:
                // Activating Warpinator requires unlocking the device first
                unlockAndRun(this::startService);
                break;
        }
    }

    void updateTile(boolean active) {
        Tile tile = getQsTile();
        if (active) {
            tile.setState(Tile.STATE_ACTIVE);
        } else {
            tile.setState(Tile.STATE_INACTIVE);
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                tile.setSubtitle(null);
            }
        }
        tile.updateTile();
    }

    private void updateBoundService(MainServiceBinder binder) {
        this.binder = binder;
        if (binder == null) {
            serviceConnection = null;
//            if (unregisterObserver != null) {
//                unregisterObserver.dispose();
//                unregisterObserver = null;
//            }
            updateTile(false);
        } else {
//            unregisterObserver = this.binder.getService().observeDeviceCount(this);
            updateTile(true);
        }
    }

    private void startService() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            try {
                startForegroundService(serviceIntent);
            } catch (Exception e) {
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
                    Log.e("Tile", "Cannot start main service from background on Android 14+", e);
                    Toast.makeText(this, "Cannot Warpinator from tile on Android 14+", Toast.LENGTH_LONG).show();
                } else Log.e("Tile", "Cannot start main service (Android <14)", e);
                return;
            }
        } else {
            startService(serviceIntent);
        }
        joinService();
    }

    private void joinService() {
        bindService(serviceIntent, serviceConnection = new ServiceConnection() {
            @Override
            public void onServiceConnected(ComponentName componentName, IBinder iBinder) {
                updateBoundService((MainServiceBinder) iBinder);
            }

            @Override
            public void onServiceDisconnected(ComponentName componentName) {
                updateBoundService(null);
            }
        }, 0);
    }

    private void stopService() {
        updateTile(false);
        updateBoundService(null);
        Intent stopIntent = new Intent(this, StopSvcReceiver.class);
        stopIntent.setAction(MainService.ACTION_STOP);
        sendBroadcast(stopIntent);
    }
}
