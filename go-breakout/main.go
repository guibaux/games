package main

import (
	"fmt"
	"image/color"
	"math/rand"

	// "github.com/guibaux/games/go-breakout/breakout"
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
)

type Block struct {
	X    float64
	Y    float64
	life int
}

type Game struct {
	bricks  []Block
	ball    *Block
	ballDir [2]int

	// Player
	X      float64
	score  int
	speed  float64
	paused bool
	life   int

	// Debug Params
	debug         bool
	printFPS      bool
	rainbowPaddle bool
}

const (
	paddleWidth  = 6
	paddleHeight = 2

	ScrWidth  = 30
	ScrHeight = 40
)
const (
	dirLeft = iota
	dirUp
	dirDown
	dirRight
)

func NewGame() *Game {
	return &Game{
		X:      (ScrWidth / 2) + (paddleWidth / 3),
		speed:  0.85,
		paused: true,
		score:  0,
		life:   3,

		bricks:  CreateBricks(),
		ball:    &Block{X: ScrWidth / 2, Y: ScrHeight / 3},
		ballDir: [2]int{dirDown, dirDown},
		// Debug
		debug:         false,
		printFPS:      false,
		rainbowPaddle: false,
	}
}

// Create bricks in a structured way
func CreateBricks() []Block {
	bricks := make([]Block, 46) // 46 = 3 Rows

	height := 2.0
	padding := 3.0
	life := 4

	bricks[0] = Block{X: padding + 3, Y: height, life: life}
	for i := 1; i < len(bricks)-1; i++ {
		if bricks[i-1].X+padding > ScrWidth {
			height += 2
			life--

			bricks[i].X = padding + 3
		} else {
			bricks[i].X = bricks[i-1].X + padding
		}
		bricks[i].Y = height
		bricks[i].life = life
	}

	return bricks
}

// Move ball to center
func (g *Game) resetBall() {
	g.ball = &Block{X: ScrWidth / 2, Y: ScrHeight / 3}
	g.ballDir[1] = dirUp
}

func (g *Game) Update() error {
	// Game over
	if g.life <= 0 {
		return nil
	}

	// Pause
	if inpututil.IsKeyJustPressed(ebiten.KeyEscape) {
		g.paused = !g.paused
	}
	if g.paused {
		return nil
	}

	// Movement
	if ebiten.IsKeyPressed(ebiten.KeyLeft) {
		if g.X > 0 {
			g.X -= g.speed
		}
	} else if ebiten.IsKeyPressed(ebiten.KeyRight) {
		if g.X < (ScrWidth + 4) {
			g.X += g.speed
		}
	}

	// Ball
	if g.ball.Y <= 0 {
		g.ballDir[1] = dirDown
	} else if g.ball.X <= 0 {
		g.ballDir[0] = dirRight
	} else if g.ball.X >= ScrWidth {
		g.ballDir[0] = dirLeft
	}

	switch g.ballDir[1] {
	case dirUp:
		g.ball.Y -= g.speed
	case dirDown:
		g.ball.Y += g.speed
	}
	switch g.ballDir[0] {
	case dirLeft:
		g.ball.X -= g.speed
	case dirRight:
		g.ball.X += g.speed
	}

	// Collision with the player
	if (g.ball.Y >= 25.0 && g.ball.Y <= 25.9) && (g.ball.X >= g.X-1 && g.ball.X <= g.X+paddleWidth) {
		g.ballDir[1] = dirUp
		nextDir := rand.Intn(2)
		if nextDir == 1 {
			g.ballDir[0] = dirLeft
		} else {
			g.ballDir[0] = dirRight
		}
	}
	// Collision with other bricks
	for i := range g.bricks {
		brick := &g.bricks[i]
		if g.ball.X >= brick.X-2 && g.ball.X <= brick.X && brick.life > 0 {
			// Bottom Collide
			if g.ball.Y >= brick.Y && g.ball.Y <= brick.Y+0.9 {
				g.ballDir[1] = dirDown
			}
			if g.ball.Y >= brick.Y+1 && g.ball.Y <= brick.Y+1.9 {
				g.ballDir[1] = dirUp
			} else {
				continue
			}
			brick.life--
			if brick.life <= 0 {
				g.score++
			}
		}
	}

	if g.ball.Y >= ScrHeight {
		g.life--
		g.resetBall()
	}

	// Debug
	if inpututil.IsKeyJustPressed(ebiten.KeyEqual) {
		g.debug = !g.debug
		fmt.Printf("Debug Mode = %t\n", g.debug)
	}
	if g.debug {
		g.debugMode()
	}
	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	// Game over
	if g.life <= 0 {
		ebitenutil.DebugPrint(screen, fmt.Sprintf("%d\nBricks", g.score))
		return
	}

	// Player
	ebitenutil.DrawRect(screen, g.X, 25, paddleWidth, paddleHeight, color.RGBA{255, 0, 0, 255})

	// Ball
	ebitenutil.DrawRect(screen, g.ball.X, g.ball.Y, 1, 1, color.RGBA{0, 0, 255, 255})
	// Bricks
	for i := range g.bricks {
		if g.bricks[i].life > 0 {
			ebitenutil.DrawRect(screen, g.bricks[i].X, g.bricks[i].Y, 2, 1, color.RGBA{0, 255, 0, 255})
		}
	}

}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return ScrHeight, ScrWidth
}

func (g *Game) debugMode() {
	if inpututil.IsKeyJustPressed(ebiten.KeyJ) {
		g.speed += 0.1
		fmt.Printf("Speed = %f", g.speed)
	} else if inpututil.IsKeyJustPressed(ebiten.KeyK) {
		g.speed -= 0.1
		fmt.Printf("Speed = %f", g.speed)
	}

	// FPS Counting
	if inpututil.IsKeyJustPressed(ebiten.KeyF) {
		g.printFPS = !g.printFPS
	}
	if g.printFPS {
		fmt.Printf("FPS: %d\n", ebiten.CurrentFPS)
	}

}

func main() {
	ebiten.SetWindowSize(640, 480)
	ebiten.SetWindowTitle("Boar-out!")

	if err := ebiten.RunGame(NewGame()); err != nil {
		fmt.Println(err)
	}
}
