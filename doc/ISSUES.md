# fui_system

- FileDialog (Qt/Linux) - when opened crashes the app on application close.

  >I'm not sure of the reason, but dialog creates new UI thread and this new thread crashes on app exit. It also prints: "kf.kio.core: Malformed JSON protocol file for protocol: "trash" , number of the ExtraNames fields should match the number of ExtraTypes fields" 
