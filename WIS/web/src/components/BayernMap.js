import React, { Component } from 'react';
import { ReactComponent as BayernMapSVG } from '../assets/Bayern_Landtagswahlkreise_2018.svg';

export default class BayernMap extends Component {
  constructor(props) {
    super(props);
    this.setUpSVG = this.setUpSVG.bind(this);

    this.state = {
      selectedElement: null,
    };
  }

  render() {
    return <BayernMapSVG id={'bayernmapsvg'} />;
  }

  componentDidMount() {
    let map = document.getElementById('bayernmapsvg');
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
      this.props.onClick(this.state.selectedElement.getAttribute('sk'));
    };
    let svgMouseOverListener = e => {
      if(e.target !== this.state.selectedElement) {
        e.target.setAttribute('fill', '#93dee2');
      }
    };
    let svgMouseOutListener = e => {
      if(e.target !== this.state.selectedElement) {
        e.target.setAttribute('fill', '#ffffff');
      }
    };

    visitChildren(svg.children, x => {
      if (x.getAttribute('sk')) {
        x.setAttribute('fill', '#ffffff'); // make it clickable
        x.onclick = svgClickListener;
        x.onmouseover = svgMouseOverListener;
        x.onmouseout = svgMouseOutListener;
      }
    });
  }
}
