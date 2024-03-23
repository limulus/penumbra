import type { HLSVideoElement } from 'hls-video-element'
import Hls from 'hls.js'
import 'hls-video-element'
import 'media-chrome'

const template = document.createElement('template')
template.innerHTML = /* HTML */ `
  <media-controller style="aspect-ratio: 16/9; width: 100%">
    <hls-video crossorigin preload="metadata" slot="media"></hls-video>
    <media-settings-menu hidden anchor="auto">
      <media-settings-menu-item>
        Speed
        <media-playback-rate-menu slot="submenu" hidden>
          <div slot="title">Speed</div>
        </media-playback-rate-menu>
      </media-settings-menu-item>
      <media-settings-menu-item>
        Quality
        <media-rendition-menu slot="submenu" hidden>
          <div slot="title">Quality</div>
        </media-rendition-menu>
      </media-settings-menu-item>
      <media-settings-menu-item>
        Captions
        <media-captions-menu slot="submenu" hidden>
          <div slot="title">Captions</div>
        </media-captions-menu>
      </media-settings-menu-item>
    </media-settings-menu>
    <media-control-bar>
      <media-play-button></media-play-button>
      <media-mute-button></media-mute-button>
      <media-volume-range></media-volume-range>
      <media-time-display showduration></media-time-display>
      <media-time-range></media-time-range>
      <media-settings-menu-button></media-settings-menu-button>
      <media-airplay-button></media-airplay-button>
      <media-fullscreen-button></media-fullscreen-button>
    </media-control-bar>
  </media-controller>

  <style>
    media-control-bar {
      backdrop-filter: blur(10px);
      -webkit-backdrop-filter: blur(10px);
    }

    media-mute-button + media-volume-range {
      width: 0;
      overflow: hidden;
      transition: width 0.2s ease-in;
    }

    /* Expand volume control in all relevant states */
    media-mute-button:hover + media-volume-range,
    media-mute-button:focus + media-volume-range,
    media-mute-button:focus-within + media-volume-range,
    media-volume-range:hover,
    media-volume-range:focus,
    media-volume-range:focus-within {
      width: 70px;
    }

    /* Do not show the airplay button unless AirPlay is available */
    media-airplay-button[mediaairplayunavailable] {
      display: none;
    }
  </style>
`

Hls.DefaultConfig.debug = console
Hls.DefaultConfig.preferManagedMediaSource = false

class VideoOnDemand extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot!.appendChild(template.content.cloneNode(true))
  }

  connectedCallback() {
    const vodUrl = `https://vod.limulus.net/${this.getAttribute('vod')}`

    const videoEl = this.shadowRoot!.querySelector<HLSVideoElement>('hls-video')!
    videoEl.setAttribute('poster', `${vodUrl}/poster.jpeg`)

    const hlsSourceEl = document.createElement('source')
    hlsSourceEl.setAttribute('type', 'application/vnd.apple.mpegurl')
    hlsSourceEl.setAttribute('src', `${vodUrl}/index.m3u8`)
    videoEl.appendChild(hlsSourceEl)

    videoEl.addEventListener('loadedmetadata', () => {
      if (!videoEl.api) return
      if (videoEl.api.levels.some((level) => level.videoCodec?.startsWith('hvc1'))) {
        videoEl.api.levels.forEach((level, index) => {
          if (level.videoCodec?.startsWith('avc1')) {
            videoEl.api?.removeLevel(index)
          }
        })
      }
    })

    videoEl.setAttribute('src', `${vodUrl}/index.m3u8`)
  }
}

if (!customElements.get('video-on-demand')) {
  customElements.define('video-on-demand', VideoOnDemand)
}
