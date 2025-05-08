using System;
using System.Runtime.CompilerServices;

namespace Rasl {
    public class Component {
        protected uint EntityId;

        public Component() {
            this.EntityId = 0;
        }

        public Component(uint id) {
            this.EntityId = id;
        }
    }

    public class Vec3 : Component {
        public float X { get; set; }
        public float Y { get; set; }
        public float Z { get; set; }

        public Vec3(float x, float y, float z) {
            this.X = x;
            this.Y = y;
            this.Z = z;
        }

        public Vec3(float v) {
            this.X = v;
            this.Y = v;
            this.Z = v;
        }
    }

    public class Transform: Component {
        public Vec3 Position { get; set; }
        public Vec3 Rotation { get; set; }
        public Vec3 Scale { get; set; }

        public Transform(Vec3 position, Vec3 rotation, Vec3 scale) {
            this.Position = position;
            this.Rotation = rotation;
            this.Scale = scale;
        }
    }

    public class Entity {
        protected uint EntityId;

        public Entity() => this.EntityId = 0;
        public Entity(uint id) => this.EntityId = id;

        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        public extern Vec3 GetTransform();
    }

    public class Script: Entity {
        public virtual void Create() {}
        public virtual void Update() {}
        public virtual void FixedUpdate(float dt) {}
        public virtual void Destroy() {}
    }
}
