'use client'

import { useState, useEffect, useActionState, useRef, Fragment } from 'react'

import { Canvas, extend, useThree, useFrame } from "@react-three/fiber";
import * as THREE from 'three';

const ThreeComponent = (props: {style: string}) => {
    //const { camera, gl, scene } = useThree();
    const { style, ...rest } = props;
    const renderRef = useRef(null);
    let renderer: any, camera: any, scene: any, controls: any;

    const [count, setCount] = useState(0);
console.log('ccccccc')
    useEffect(() => {
        console.log('aaaaaaaaaa')
        if (renderRef.current) {
          console.log('bbbbbb')
            // 初始化 Three.js
            renderer = new THREE.WebGLRenderer({ antialias: true });
            renderer.setSize(renderRef.current.clientWidth, renderRef.current.clientHeight);
            renderRef.current.appendChild(renderer.domElement);
            console.log('dddddddd')
            // 初始化相机
            camera = new THREE.PerspectiveCamera(75, renderRef.current.clientWidth / renderRef.current.clientHeight, 0.1, 1000);
            camera.position.set(0, 0, 5);

            // 初始化场景
            scene = new THREE.Scene();

            // 添加光源
            const light = new THREE.HemisphereLight(0xffffff, 0x444444);
            light.position.set(0, 200, 0);
            scene.add(light);

            // 添加立方体
            const geometry = new THREE.BoxGeometry();
            const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
            const cube = new THREE.Mesh(geometry, material);
            scene.add(cube);

            // 渲染函数
            const animate = () => {
                requestAnimationFrame(animate);
                cube.rotation.x += 0.01;
                cube.rotation.y += 0.01;
                renderer.render(scene, camera);
            };

            animate();
        }
    }, [/*count*/]);

    return (
    <div>
      <div ref={renderRef} style={{ width: '100%', height: '100%', ...style }} {...rest}></div>
      <button onClick={() => setCount(count + 1)}>Increment</button>
    </div>
    );
}

function MyAnimatedBox() {
  const myMesh = useRef()

  useFrame(({ clock }) => {
    const a = clock.elapsedTime
    //console.log(a)
    myMesh.current.rotation.x = a
  })

  return (
    <mesh ref={myMesh}>
      <boxGeometry args={[2, 2, 2]} />
      <meshBasicMaterial color="royalblue" />
    </mesh>
  )
}

export default function Page() {
    const [company, setCompany] = useState("")

    useEffect(() => {
        console.log('11111111')
    }, []);

    return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">three.js</h1>
          <Canvas>
            <MyAnimatedBox />
            <ambientLight intensity={0.1} />
            <directionalLight position={[0, 0, 5]} color="red" />
          </Canvas>
          {/*<ThreeComponent style="" />*/}
        </div>
      </main>
      <footer className="flex flex-col items-center justify-items-center gap-2">
        <div>
        </div>
      </footer>
    </div>
  );
}