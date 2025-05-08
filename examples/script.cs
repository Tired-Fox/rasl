using System;
using Rasl;

class CustomScript: Script {

    Vec3 pos;

    void Update() {
        Console.WriteLine("[" + this.EntityId + "]" + " BevyScript Update: ");
        Console.WriteLine(this.pos);
    }
}
