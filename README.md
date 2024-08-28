# waveos
Audio and Sound control platform aims to provide an OSC based interface to interact/control audio capable devices

#

<div hidden>
```
@startuml wavesOS_system
!define SPRITESURL https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master
!define ICONURL https://raw.githubusercontent.com/Roemer/plantuml-office/master/icons

!includeurl SPRITESURL/C4_Context.puml
!includeurl SPRITESURL/C4_Container.puml

' LAYOUT_TOP_DOWN()
' LAYOUT_AS_SKETCH()
LAYOUT_WITH_LEGEND()

title System Context diagram for WavesOS System

Person_Ext(soundUser, "Sound User", "Controls sound")
System(soundSystem, "Sound Control Hub", "Processes sound control signals [OSC]")
Container(audioController, "Audio Control", "Manages audio settings", "audio-controller.png")
Container(soundEngine, "Sound Control Engine", "Generates sound effects", "sound-engine.png")
Container(audioInterface, "Audio Interface Control", "Audio Device Connectivity", "audio-device.png")

Rel(soundUser, soundSystem, "Communicates", "OSC over UDP/HTTP")

Rel(soundSystem, audioController, "Amplification/ Signal Routing and Mixing / EQ")
Rel(soundSystem, soundEngine, "Generates sound")
BiRel(audioController, audioInterface, "IO")
BiRel(audioInterface, soundEngine, "IO")
@enduml
```
</div>

![](wavesOS_system.svg)