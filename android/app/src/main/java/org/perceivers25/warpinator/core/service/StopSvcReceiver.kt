package org.perceivers25.warpinator.core.service

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import dagger.hilt.android.AndroidEntryPoint
import org.perceivers25.warpinator.core.data.ServiceState
import org.perceivers25.warpinator.core.data.WarpinatorRepository
import javax.inject.Inject

@AndroidEntryPoint
class StopSvcReceiver : BroadcastReceiver() {
    @Inject
    lateinit var repository: WarpinatorRepository
    override fun onReceive(context: Context, intent: Intent) {
        if (MainService.ACTION_STOP == intent.action) {
            context.stopService(Intent(context, MainService::class.java))
            repository.updateServiceState(ServiceState.Stopped)
        }
    }
}