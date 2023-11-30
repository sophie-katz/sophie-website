import * as PIXI from "pixi.js";

export class Viewport {
    public offset: PIXI.Point = new PIXI.Point(0, 0);
    public zoom: number = 1.0;
}
