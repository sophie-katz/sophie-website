import * as PIXI from 'pixi.js';
import type { Viewport } from './viewport';

export class Layer {
    private readonly graphics: PIXI.Graphics = new PIXI.Graphics();
    private readonly sprite: PIXI.Sprite = new PIXI.Sprite();
    private counter: number = 0;

    constructor(private readonly renderer: PIXI.IRenderer<PIXI.ICanvas>, private readonly viewport: Viewport, private readonly size: {width: number, height: number}) {
        this.graphics.beginFill(0xffffff);
		this.graphics.drawRect(0, 0, this.size.width, this.size.height);
		this.graphics.endFill();

        this.sprite.eventMode = "static";
        // this.sprite.on("pointerdown", (event: PIXI.FederatedPointerEvent): void => {
        //     const {x, y} = event.getLocalPosition(this.sprite);
        //     this.update(Math.floor(x), Math.floor(y));
        //     this.render();
        // });
        this.sprite.on("pointermove", (event: PIXI.FederatedPointerEvent): void => {
            if (event.buttons === 1) {
                const {x, y} = event.getLocalPosition(this.sprite);
                this.update(Math.floor(x), Math.floor(y));
                this.render();
            }
        });
    }

    public mount(container: PIXI.Container) {
        container.addChild(this.sprite);
        this.render();
        this.navigate();
    }

    public unmount(container: PIXI.Container) {
        container.removeChild(this.sprite);
    }

    public update(x: number, y: number) {
        if (this.counter % 2 == 0) {
            this.graphics.beginFill(0x0000ff);
        } else {
            this.graphics.beginFill(0xff0000);
        }
        this.counter++;
		this.graphics.drawRect(x, y, 1, 1);
		this.graphics.endFill();
    }

    public render() {
        const texture = this.renderer.generateTexture(this.graphics);
        texture.baseTexture.scaleMode = PIXI.SCALE_MODES.NEAREST;
        this.sprite.texture = texture;
    }

    public navigate() {
        this.sprite.x = this.viewport.offset.x;
        this.sprite.y = this.viewport.offset.y;
        this.sprite.scale.set(this.viewport.zoom);
    }
};
