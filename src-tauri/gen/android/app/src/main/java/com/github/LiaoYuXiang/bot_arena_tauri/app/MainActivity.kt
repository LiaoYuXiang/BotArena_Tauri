package com.github.LiaoYuXiang.bot_arena_tauri.app

import android.os.Bundle
//import com.github.LiaoYuXiang.bot_arena_tauri.app.databinding.MainActivityBinding
//import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import android.view.WindowManager
import android.view.WindowInsets

class MainActivity : TauriActivity() {

//  private lateinit var binding: MainActivityBinding

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    // 如果你用 ViewBinding，可以這樣設
    // binding = MainActivityBinding.inflate(layoutInflater)
    // setContentView(binding.root)

    // 否則直接用 setContentView 設定 layout
    setContentView(R.layout.activity_main)

    // 讓內容延伸到系統 UI（例如瀏海區）
    WindowCompat.setDecorFitsSystemWindows(window, false)

    // 隱藏狀態列與導覽列（全螢幕）
    val controller = WindowInsetsControllerCompat(window, window.decorView)
    controller.systemBarsBehavior =
      WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    controller.hide(WindowInsetsCompat.Type.systemBars())
  }
}
