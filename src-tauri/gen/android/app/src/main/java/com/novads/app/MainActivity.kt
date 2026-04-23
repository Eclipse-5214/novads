package com.novads.app

import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.net.wifi.WifiManager
import android.os.Bundle
import android.util.Log
import android.view.WindowInsets
import android.view.WindowInsetsController
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {

  private lateinit var connectivityManager: ConnectivityManager
  private var wifiNetworkCallback: ConnectivityManager.NetworkCallback? = null
  private var multicastLock: WifiManager.MulticastLock? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    bindToWifi()
    acquireMulticastLock()
    super.onCreate(savedInstanceState)
    hideSystemUI()
  }

  private fun hideSystemUI() {
    window.insetsController?.let {
      it.hide(WindowInsets.Type.statusBars() or WindowInsets.Type.navigationBars())
      it.systemBarsBehavior = WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }
  }

  private fun bindToWifi() {
    connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager

    val request = NetworkRequest.Builder()
      .addTransportType(NetworkCapabilities.TRANSPORT_WIFI)
      .build()

    wifiNetworkCallback = object : ConnectivityManager.NetworkCallback() {
      override fun onAvailable(network: Network) {
        Log.d("NovaDS", "WiFi network available, binding process")
        connectivityManager.bindProcessToNetwork(network)
      }

      override fun onLost(network: Network) {
        Log.d("NovaDS", "WiFi network lost, clearing binding")
        connectivityManager.bindProcessToNetwork(null)
      }
    }

    connectivityManager.requestNetwork(request, wifiNetworkCallback!!)

    // Also try to bind immediately if WiFi is already connected
    val activeNetwork = connectivityManager.activeNetwork
    val caps = activeNetwork?.let { connectivityManager.getNetworkCapabilities(it) }
    if (caps != null && caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)) {
      Log.d("NovaDS", "Already on WiFi, binding immediately")
      connectivityManager.bindProcessToNetwork(activeNetwork)
    }
  }

  private fun acquireMulticastLock() {
    val wifiManager = applicationContext.getSystemService(Context.WIFI_SERVICE) as WifiManager
    multicastLock = wifiManager.createMulticastLock("novads_robot_comms")
    multicastLock?.setReferenceCounted(false)
    multicastLock?.acquire()
    Log.d("NovaDS", "MulticastLock acquired — inbound UDP enabled")
  }

  override fun onDestroy() {
    super.onDestroy()
    multicastLock?.release()
    wifiNetworkCallback?.let { connectivityManager.unregisterNetworkCallback(it) }
    connectivityManager.bindProcessToNetwork(null)
  }
}
