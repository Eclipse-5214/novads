package com.novads.app

import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.os.Bundle
import android.util.Log
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {

  private lateinit var connectivityManager: ConnectivityManager
  private var wifiNetworkCallback: ConnectivityManager.NetworkCallback? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    bindToWifi()
    super.onCreate(savedInstanceState)
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

  override fun onDestroy() {
    super.onDestroy()
    wifiNetworkCallback?.let { connectivityManager.unregisterNetworkCallback(it) }
    connectivityManager.bindProcessToNetwork(null)
  }
}
