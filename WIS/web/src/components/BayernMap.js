import React, { Component } from 'react';
import { ReactComponent as BayernMapSVG } from '../assets/Bayern_Landtagswahlkreise_2018.svg';

/**
 * Required props:
 *  - onClick: handler for handling click events on either Wahlkreise or Stimmkreise.
 *    Has the ID of the Stimmkreis or Wahlkreis as an argument.
 *  - mode: can either be 'wk' for selecting Wahlkreise or 'sk' for selecting Stimmkreise.
 */
export default class BayernMap extends Component {
  constructor(props) {
    super(props);
    this.setUpSVG = this.setUpSVG.bind(this);
    this.mapcanvas = React.createRef(); // can't ref directly on SVG...

    this.state = {
      selectedElement: null,
    };
  }

  render() {
    return (
      <div ref={this.mapcanvas} className="bayern-map">
        <BayernMapSVG />
      </div>
    );
  }

  componentDidMount() {
    let map = this.mapcanvas.current.children[0];
    this.setUpSVG(map);
  }

  setUpSVG(svg) {
    function visitChildren(children, callback) {
      for (let i = 0; i < children.length; i++) {
        if (children[i].children.length) {
          visitChildren(children[i].children, callback);
        } else {
          callback(children[i]);
        }
      }
    }

    let svgClickListener = e => {
      // reset old selection
      if (this.state.selectedElement) {
        this.state.selectedElement.setAttribute('fill', '#ffffff');
      }

      // update new selection
      this.setState({ selectedElement: e.target });
      this.state.selectedElement.setAttribute('fill', '#09b6bf');

      // trigger callback
      this.props.onClick(parseInt(this.state.selectedElement.getAttribute(this.props.mode)));
    };
    let svgMouseOverListener = e => {
      if (e.target !== this.state.selectedElement) {
        e.target.setAttribute('fill', '#93dee2');
      }
    };
    let svgMouseOutListener = e => {
      if (e.target !== this.state.selectedElement) {
        e.target.setAttribute('fill', '#ffffff');
      }
    };

    visitChildren(svg.children, x => {
      if (x.getAttribute(this.props.mode)) {
        x.setAttribute('fill', '#ffffff'); // make it clickable
        x.onclick = svgClickListener;
        x.onmouseover = svgMouseOverListener;
        x.onmouseout = svgMouseOutListener;
      } else {
        // make everything else including labels 'invisible' for mouse events
        x.style['pointer-events'] = 'none';
      }
    });
  }
}
